// SRC: ../rust/compiler/rustc_expand/src/errors.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
use std::borrow::Cow;

use crate::rustc_complete::ast;
use crate::rustc_complete::codes::*;
use crate::rustc_complete::limit::Limit;
use rustc_macros::{Diagnostic, Subdiagnostic};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Ident, MacroRulesNormalizedIdent, Span, Symbol};
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_expr_repeat_no_syntax_vars)]
pub(crate) struct NoSyntaxVarsExprRepeat {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_must_repeat_once)]
pub(crate) struct MustRepeatOnce {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_count_repetition_misplaced)]
pub(crate) struct CountRepetitionMisplaced {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_var_still_repeating)]
pub(crate) struct VarStillRepeating {
    #[primary_span]
    pub span: Span,
    pub ident: MacroRulesNormalizedIdent,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_meta_var_dif_seq_matchers)]
pub(crate) struct MetaVarsDifSeqMatchers {
    #[primary_span]
    pub span: Span,
    pub msg: String,
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_resolve_relative_path)]
pub(crate) struct ResolveRelativePath {
    #[primary_span]
    pub span: Span,
    pub path: String,
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_collapse_debuginfo_illegal)]
pub(crate) struct CollapseMacroDebuginfoIllegal {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(expand_macro_const_stability)]
pub(crate) struct MacroConstStability {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(expand_label2)]
    pub head_span: Span,
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(expand_macro_body_stability)]
pub(crate) struct MacroBodyStability {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(expand_label2)]
    pub head_span: Span,
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */

#[derive(Diagnostic)]
#[diag(expand_feature_removed, code = E0557)]
#[note]
pub(crate) struct FeatureRemoved<'a> {
    #[primary_span]
    #[label]
    pub span: Span,
    #[subdiagnostic]
    pub reason: Option<FeatureRemovedReason<'a>>,
    pub removed_rustc_version: &'a str,
    pub pull_note: String,
}
/* AST_META: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Subdiagnostic)]
#[note(expand_reason)]
pub(crate) struct FeatureRemovedReason<'a> {
    pub reason: &'a str,
}
/* AST_META: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_feature_not_allowed, code = E0725)]
pub(crate) struct FeatureNotAllowed {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}
/* AST_META: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(expand_recursion_limit_reached)]
#[help]
pub(crate) struct RecursionLimitReached {
    #[primary_span]
    pub span: Span,
    pub descr: String,
    pub suggested_limit: Limit,
    pub crate_name: Symbol,
}
/* AST_META: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(expand_malformed_feature_attribute, code = E0556)]
pub(crate) struct MalformedFeatureAttribute {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub help: MalformedFeatureAttributeHelp,
}
/* AST_META: AST_ID=17 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=15 */

#[derive(Subdiagnostic)]
pub(crate) enum MalformedFeatureAttributeHelp {
    #[label(expand_expected)]
    Label {
        #[primary_span]
        span: Span,
    },
    #[suggestion(expand_expected, code = "{suggestion}", applicability = "maybe-incorrect")]
    Suggestion {
        #[primary_span]
        span: Span,
        suggestion: Symbol,
    },
}
/* AST_META: AST_ID=18 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_remove_expr_not_supported)]
pub(crate) struct RemoveExprNotSupported {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=19 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=34 */

#[derive(Diagnostic)]
pub(crate) enum InvalidCfg {
    #[diag(expand_invalid_cfg_no_parens)]
    NotFollowedByParens {
        #[primary_span]
        #[suggestion(
            expand_invalid_cfg_expected_syntax,
            code = "cfg(/* predicate */)",
            applicability = "has-placeholders"
        )]
        span: Span,
    },
    #[diag(expand_invalid_cfg_no_predicate)]
    NoPredicate {
        #[primary_span]
        #[suggestion(
            expand_invalid_cfg_expected_syntax,
            code = "cfg(/* predicate */)",
            applicability = "has-placeholders"
        )]
        span: Span,
    },
    #[diag(expand_invalid_cfg_multiple_predicates)]
    MultiplePredicates {
        #[primary_span]
        span: Span,
    },
    #[diag(expand_invalid_cfg_predicate_literal)]
    PredicateLiteral {
        #[primary_span]
        span: Span,
    },
}
/* AST_META: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(expand_wrong_fragment_kind)]
pub(crate) struct WrongFragmentKind<'a> {
    #[primary_span]
    pub span: Span,
    pub kind: &'a str,
    pub name: &'a ast::Path,
}
/* AST_META: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_unsupported_key_value)]
pub(crate) struct UnsupportedKeyValue {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=22 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=23 */

#[derive(Diagnostic)]
#[diag(expand_incomplete_parse)]
#[note]
pub(crate) struct IncompleteParse<'a> {
    #[primary_span]
    pub span: Span,
    pub descr: String,
    #[label]
    pub label_span: Span,
    pub macro_path: &'a ast::Path,
    pub kind_name: &'a str,
    #[note(expand_macro_expands_to_match_arm)]
    pub expands_to_match_arm: bool,

    #[suggestion(
        expand_suggestion_add_semi,
        style = "verbose",
        code = ";",
        applicability = "maybe-incorrect"
    )]
    pub add_semicolon: Option<Span>,
}
/* AST_META: AST_ID=23 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_remove_node_not_supported)]
pub(crate) struct RemoveNodeNotSupported {
    #[primary_span]
    pub span: Span,
    pub descr: &'static str,
}
/* AST_META: AST_ID=24 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_module_circular)]
pub(crate) struct ModuleCircular {
    #[primary_span]
    pub span: Span,
    pub modules: String,
}
/* AST_META: AST_ID=25 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(expand_module_in_block)]
pub(crate) struct ModuleInBlock {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub name: Option<ModuleInBlockName>,
}
/* AST_META: AST_ID=26 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Subdiagnostic)]
#[note(expand_note)]
pub(crate) struct ModuleInBlockName {
    #[primary_span]
    pub span: Span,
    pub name: Ident,
}
/* AST_META: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */

#[derive(Diagnostic)]
#[diag(expand_module_file_not_found, code = E0583)]
#[help]
#[note]
pub(crate) struct ModuleFileNotFound {
    #[primary_span]
    pub span: Span,
    pub name: Ident,
    pub default_path: String,
    pub secondary_path: String,
}
/* AST_META: AST_ID=28 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Diagnostic)]
#[diag(expand_module_multiple_candidates, code = E0761)]
#[help]
pub(crate) struct ModuleMultipleCandidates {
    #[primary_span]
    pub span: Span,
    pub name: Ident,
    pub default_path: String,
    pub secondary_path: String,
}
/* AST_META: AST_ID=29 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_trace_macro)]
pub(crate) struct TraceMacro {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=30 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(expand_proc_macro_panicked)]
pub(crate) struct ProcMacroPanicked {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub message: Option<ProcMacroPanickedHelp>,
}
/* AST_META: AST_ID=31 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Subdiagnostic)]
#[help(expand_help)]
pub(crate) struct ProcMacroPanickedHelp {
    pub message: String,
}
/* AST_META: AST_ID=32 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(expand_proc_macro_derive_panicked)]
pub(crate) struct ProcMacroDerivePanicked {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub message: Option<ProcMacroDerivePanickedHelp>,
}
/* AST_META: AST_ID=33 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Subdiagnostic)]
#[help(expand_help)]
pub(crate) struct ProcMacroDerivePanickedHelp {
    pub message: String,
}
/* AST_META: AST_ID=34 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(expand_custom_attribute_panicked)]
pub(crate) struct CustomAttributePanicked {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub message: Option<CustomAttributePanickedHelp>,
}
/* AST_META: AST_ID=35 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Subdiagnostic)]
#[help(expand_help)]
pub(crate) struct CustomAttributePanickedHelp {
    pub message: String,
}
/* AST_META: AST_ID=36 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_proc_macro_derive_tokens)]
pub(crate) struct ProcMacroDeriveTokens {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=37 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(expand_duplicate_matcher_binding)]
pub(crate) struct DuplicateMatcherBinding {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(expand_label2)]
    pub prev: Span,
}
/* AST_META: AST_ID=38 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=17 */

#[derive(Diagnostic)]
#[diag(expand_missing_fragment_specifier)]
#[note]
#[help(expand_valid)]
pub(crate) struct MissingFragmentSpecifier {
    #[primary_span]
    pub span: Span,
    #[suggestion(
        expand_suggestion_add_fragspec,
        style = "verbose",
        code = ":spec",
        applicability = "maybe-incorrect"
    )]
    pub add_span: Span,
    pub valid: &'static str,
}
/* AST_META: AST_ID=39 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(expand_invalid_fragment_specifier)]
#[help]
pub(crate) struct InvalidFragmentSpecifier {
    #[primary_span]
    pub span: Span,
    pub fragment: Ident,
    pub help: &'static str,
}
/* AST_META: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_expected_paren_or_brace)]
pub(crate) struct ExpectedParenOrBrace<'a> {
    #[primary_span]
    pub span: Span,
    pub token: Cow<'a, str>,
}
/* AST_META: AST_ID=41 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(expand_empty_delegation_mac)]
pub(crate) struct EmptyDelegationMac {
    #[primary_span]
    pub span: Span,
    pub kind: String,
}
/* AST_META: AST_ID=42 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_glob_delegation_outside_impls)]
pub(crate) struct GlobDelegationOutsideImpls {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=43 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_crate_name_in_cfg_attr)]
pub(crate) struct CrateNameInCfgAttr {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=44 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_crate_type_in_cfg_attr)]
pub(crate) struct CrateTypeInCfgAttr {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=45 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(expand_glob_delegation_traitless_qpath)]
pub(crate) struct GlobDelegationTraitlessQpath {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=46 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

// This used to be the `proc_macro_back_compat` lint (#83125). It was later
// turned into a hard error.
#[derive(Diagnostic)]
#[diag(expand_proc_macro_back_compat)]
#[note]
pub(crate) struct ProcMacroBackCompat {
    pub crate_name: String,
    pub fixed_version: String,
}
/* AST_META: AST_ID=47 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=13 | LINES=57 */

pub(crate) use metavar_exprs::*;
mod metavar_exprs {
    use super::*;

    #[derive(Diagnostic, Default)]
    #[diag(expand_mve_extra_tokens)]
    pub(crate) struct MveExtraTokens {
        #[primary_span]
        #[suggestion(code = "", applicability = "machine-applicable")]
        pub span: Span,
        #[label]
        pub ident_span: Span,
        pub extra_count: usize,

        // The rest is only used for specific diagnostics and can be default if neither
        // `note` is `Some`.
        #[note(expand_exact)]
        pub exact_args_note: Option<()>,
        #[note(expand_range)]
        pub range_args_note: Option<()>,
        pub min_or_exact_args: usize,
        pub max_args: usize,
        pub name: String,
    }

    #[derive(Diagnostic)]
    #[note]
    #[diag(expand_mve_missing_paren)]
    pub(crate) struct MveMissingParen {
        #[primary_span]
        #[label]
        pub ident_span: Span,
        #[label(expand_unexpected)]
        pub unexpected_span: Option<Span>,
        #[suggestion(code = "( /* ... */ )", applicability = "has-placeholders")]
        pub insert_span: Option<Span>,
    }

    #[derive(Diagnostic)]
    #[note]
    #[diag(expand_mve_unrecognized_expr)]
    pub(crate) struct MveUnrecognizedExpr {
        #[primary_span]
        #[label]
        pub span: Span,
        pub valid_expr_list: &'static str,
    }

    #[derive(Diagnostic)]
    #[diag(expand_mve_unrecognized_var)]
    pub(crate) struct MveUnrecognizedVar {
        #[primary_span]
        pub span: Span,
        pub key: MacroRulesNormalizedIdent,
    }
}
/* AST_META: AST_ID=48 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(expand_macro_args_bad_delim)]
pub(crate) struct MacroArgsBadDelim {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub sugg: MacroArgsBadDelimSugg,
    pub rule_kw: Symbol,
}
/* AST_META: AST_ID=49 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Subdiagnostic)]
#[multipart_suggestion(expand_macro_args_bad_delim_sugg, applicability = "machine-applicable")]
pub(crate) struct MacroArgsBadDelimSugg {
    #[suggestion_part(code = "(")]
    pub open: Span,
    #[suggestion_part(code = ")")]
    pub close: Span,
}