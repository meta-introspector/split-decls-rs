use std::sync::Arc;

use rustc_ast::tokenstream::TokenStream;
use rustc_errors::ErrorGuaranteed;
use rustc_span::{Ident, MultiSpan, Span};
use rustc_span::hygiene::Transparency;
use rustc_hir::def::MacroKinds;
use rustc_ast as ast;
use rustc_session::Session;

// Local lib-macro-rule imports
use crate::{MacroRule, Tracker, NoopTracker, try_match_macro_derive, try_match_macro_attr, try_match_macro};
use crate::macro_rules_utils::{trace_macros_note_expander, is_defined_in_current_crate, has_compile_error_macro};
use crate::mbe::transcribe;
use crate::diagnostics::{FailedMacro, failed_to_match_macro};
use crate::base::{ExtCtxt, MacroExpanderResult, TTMacroExpander, AttrProcMacro, BangProcMacro, DummyResult};
use crate::mbe_macro_parser::NamedMatches;


// External Rustc imports needed
use rustc_expand_base_lib::prelude::{MacroKindTrait, OpaqueDeriveResolution};
use rustc_ast_pretty::pprust;


pub struct MacroRulesMacroExpander<DRT: OpaqueDeriveResolution + 'static> {
    pub node_id: ast::NodeId,
    pub name: Ident,
    pub span: Span,
    pub transparency: Transparency,
    pub kinds: MacroKinds,
    pub rules: Vec<MacroRule>,
}

impl<DRT: OpaqueDeriveResolution + 'static> MacroKindTrait for MacroRulesMacroExpander<DRT> {
    fn get_name(&self) -> String {
        "macro_rules".to_string()
    }
}


impl<DRT: OpaqueDeriveResolution + 'static> MacroRulesMacroExpander<DRT> {
    pub fn get_unused_rule(&self, rule_i: usize) -> Option<(&Ident, MultiSpan)> {
        // If the rhs contains an invocation like `compile_error!`, don't report it as unused.
        let (span, rhs) = match self.rules[rule_i] {
            MacroRule::Func { lhs_span, ref rhs, .. } => (MultiSpan::from_span(lhs_span), rhs),
            MacroRule::Attr { args_span, body_span, ref rhs, .. } => {
                (MultiSpan::from_spans(vec![args_span, body_span]), rhs)
            }
            MacroRule::Derive { body_span, ref rhs, .. } => (MultiSpan::from_span(body_span), rhs),
        };
        if has_compile_error_macro(rhs) { None } else { Some((&self.name, span)) }
    }

    pub fn kinds(&self) -> MacroKinds {
        self.kinds
    }

    pub fn expand_derive(
        &self,
        cx: &mut ExtCtxt<'_, DRT>,
        sp: Span,
        body: &TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        // This is similar to `expand_macro`, but they have very different signatures, and will
        // diverge further once derives support arguments.
        let Self { name, ref rules, node_id, .. } = *self;
        let psess = &cx.sess.psess;

        if cx.trace_macros() {
            let msg = format!("expanding `#[derive({name})] {}`", pprust::tts_to_string(body));
            trace_macros_note_expander(&mut cx.expansions, sp, msg);
        }

        match try_match_macro_derive(psess, name, body, rules, &mut NoopTracker) {
            Ok((rule_index, rule, named_matches)) => {
                let MacroRule::Derive { rhs, .. } = rule else {
                    panic!("try_match_macro_derive returned non-derive rule");
                };
                let Delimited(rhs_span, _, rhs) = rhs else { // This needs to be `lib_token_tree::Delimited`
                    cx.dcx().span_bug(sp, "malformed macro derive rhs");
                };

                let id = cx.current_expansion.id;
                let tts = transcribe::transcribe(psess, &named_matches, rhs, *rhs_span, self.transparency, id)
                    .map_err(|e| e.emit())?;

                if cx.trace_macros() {
                    let msg = format!("to `{}`", pprust::tts_to_string(&tts));
                    trace_macros_note_expander(&mut cx.expansions, sp, msg);
                }

                if is_defined_in_current_crate(node_id) {
                    // `record_macro_rule_usage` is no longer available.
                }

                Ok(tts)
            }
            crate::CanRetry::No(guar) => Err(guar),
            crate::CanRetry::Yes => {
                let (_, guar) = failed_to_match_macro(
                    cx.psess(),
                    sp,
                    self.span,
                    name,
                    FailedMacro::Derive,
                    body,
                    rules,
                );
                cx.macro_error_and_trace_macros_diag();
                Err(guar)
            }
        }
    }
}

impl<DRT: OpaqueDeriveResolution + 'static> TTMacroExpander<DRT> for MacroRulesMacroExpander<DRT> {
    fn expand<'cx>(
        &self,
        cx: &'cx mut ExtCtxt<'_, DRT>,
        sp: Span,
        input: TokenStream,
    ) -> MacroExpanderResult<'cx, DRT> {
        // This will need to be `crate::expand_macro`
        unimplemented!()
    }
}

impl<DRT: OpaqueDeriveResolution + 'static> AttrProcMacro<DRT> for MacroRulesMacroExpander<DRT> {
    fn expand(
        &self,
        _cx: &mut ExtCtxt<'_, DRT>,
        _sp: Span,
        _args: TokenStream,
        _body: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        unreachable!("`expand` called on `MacroRulesMacroExpander`, expected `expand_with_safety`")
    }

    fn expand_with_safety(
        &self,
        cx: &mut ExtCtxt<'_, DRT>,
        safety: ast::Safety,
        sp: Span,
        args: TokenStream,
        body: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        // This will need to be `crate::expand_macro_attr`
        unimplemented!()
    }
}
