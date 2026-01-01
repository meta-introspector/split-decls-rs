// SRC: ../rust/compiler/rustc_mir_transform/src/errors.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::codes::*;
use crate::rustc_complete::{Diag, LintDiagnostic};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use crate::rustc_complete::mir::AssertKind;
use crate::rustc_complete::query::Key;
use crate::rustc_complete::ty::TyCtxt;
use crate::rustc_complete::lint::{self, Lint};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::def_id::DefId;
use crate::rustc_complete::{Ident, Span, Symbol};
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=42 */

use crate::fluent_generated as fluent;

/// Emit diagnostic for calls to `#[inline(always)]`-annotated functions with a
/// `#[target_feature]` attribute where the caller enables a different set of target features.
pub(crate) fn emit_inline_always_target_feature_diagnostic<'a, 'tcx>(
    tcx: TyCtxt<'tcx>,
    call_span: Span,
    callee_def_id: DefId,
    caller_def_id: DefId,
    callee_only: &[&'a str],
) {
    let callee = tcx.def_path_str(callee_def_id);
    let caller = tcx.def_path_str(caller_def_id);

    tcx.node_span_lint(
        lint::builtin::INLINE_ALWAYS_MISMATCHING_TARGET_FEATURES,
        tcx.local_def_id_to_hir_id(caller_def_id.as_local().unwrap()),
        call_span,
        |lint| {
            lint.primary_message(format!(
                "call to `#[inline(always)]`-annotated `{callee}` \
                requires the same target features to be inlined"
            ));
            lint.note("function will not be inlined");

            lint.note(format!(
                "the following target features are on `{callee}` but missing from `{caller}`: {}",
                callee_only.join(", ")
            ));
            lint.span_note(callee_def_id.default_span(tcx), format!("`{callee}` is defined here"));

            let feats = callee_only.join(",");
            lint.span_suggestion(
                tcx.def_span(caller_def_id).shrink_to_lo(),
                format!("add `#[target_feature]` attribute to `{caller}`"),
                format!("#[target_feature(enable = \"{feats}\")]\n"),
                lint::Applicability::MaybeIncorrect,
            );
        },
    );
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(LintDiagnostic)]
#[diag(mir_transform_unconditional_recursion)]
#[help]
pub(crate) struct UnconditionalRecursion {
    #[label]
    pub(crate) span: Span,
    #[label(mir_transform_unconditional_recursion_call_site_label)]
    pub(crate) call_sites: Vec<Span>,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */

#[derive(Diagnostic)]
#[diag(mir_transform_force_inline_attr)]
#[note]
pub(crate) struct InvalidForceInline {
    #[primary_span]
    pub attr_span: Span,
    #[label(mir_transform_callee)]
    pub callee_span: Span,
    pub callee: String,
    pub reason: &'static str,
}
/* AST_META: AST_ID=8 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=19 */

#[derive(LintDiagnostic)]
pub(crate) enum ConstMutate {
    #[diag(mir_transform_const_modify)]
    #[note]
    Modify {
        #[note(mir_transform_const_defined_here)]
        konst: Span,
    },
    #[diag(mir_transform_const_mut_borrow)]
    #[note]
    #[note(mir_transform_note2)]
    MutBorrow {
        #[note(mir_transform_note3)]
        method_call: Option<Span>,
        #[note(mir_transform_const_defined_here)]
        konst: Span,
    },
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(mir_transform_unaligned_packed_ref, code = E0793)]
#[note]
#[note(mir_transform_note_ub)]
#[help]
pub(crate) struct UnalignedPackedRef {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Diagnostic)]
#[diag(mir_transform_unknown_pass_name)]
pub(crate) struct UnknownPassName<'a> {
    pub(crate) name: &'a str,
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

pub(crate) struct AssertLint<P> {
    pub span: Span,
    pub assert_kind: AssertKind<P>,
    pub lint_kind: AssertLintKind,
}
/* AST_META: AST_ID=12 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

pub(crate) enum AssertLintKind {
    ArithmeticOverflow,
    UnconditionalPanic,
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=decorate_lint | COMPLEXITY=11 | LINES=14 */

impl<'a, P: std::fmt::Debug> LintDiagnostic<'a, ()> for AssertLint<P> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.primary_message(match self.lint_kind {
            AssertLintKind::ArithmeticOverflow => fluent::mir_transform_arithmetic_overflow,
            AssertLintKind::UnconditionalPanic => fluent::mir_transform_operation_will_panic,
        });
        let label = self.assert_kind.diagnostic_message();
        self.assert_kind.add_args(&mut |name, value| {
            diag.arg(name, value);
        });
        diag.span_label(self.span, label);
    }
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=9 */

impl AssertLintKind {
    pub(crate) fn lint(&self) -> &'static Lint {
        match self {
            AssertLintKind::ArithmeticOverflow => lint::builtin::ARITHMETIC_OVERFLOW,
            AssertLintKind::UnconditionalPanic => lint::builtin::UNCONDITIONAL_PANIC,
        }
    }
}
/* AST_META: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(LintDiagnostic)]
#[diag(mir_transform_ffi_unwind_call)]
pub(crate) struct FfiUnwindCall {
    #[label(mir_transform_ffi_unwind_call)]
    pub span: Span,
    pub foreign: bool,
}
/* AST_META: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=9 */

#[derive(LintDiagnostic)]
#[diag(mir_transform_fn_item_ref)]
pub(crate) struct FnItemRef {
    #[suggestion(code = "{sugg}", applicability = "unspecified")]
    pub span: Span,
    pub sugg: String,
    pub ident: Ident,
}
/* AST_META: AST_ID=17 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

pub(crate) struct MustNotSupend<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub yield_sp: Span,
    pub reason: Option<MustNotSuspendReason>,
    pub src_sp: Span,
    pub pre: &'a str,
    pub def_id: DefId,
    pub post: &'a str,
}
/* AST_META: AST_ID=18 | TYPE=FUNCTION | NAME=decorate_lint | COMPLEXITY=11 | LINES=15 */

// Needed for def_path_str
impl<'a> LintDiagnostic<'a, ()> for MustNotSupend<'_, '_> {
    fn decorate_lint<'b>(self, diag: &'b mut crate::rustc_errors::Diag<'a, ()>) {
        diag.primary_message(fluent::mir_transform_must_not_suspend);
        diag.span_label(self.yield_sp, fluent::_subdiag::label);
        if let Some(reason) = self.reason {
            diag.subdiagnostic(reason);
        }
        diag.span_help(self.src_sp, fluent::_subdiag::help);
        diag.arg("pre", self.pre);
        diag.arg("def_path", self.tcx.def_path_str(self.def_id));
        diag.arg("post", self.post);
    }
}
/* AST_META: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Subdiagnostic)]
#[note(mir_transform_note)]
pub(crate) struct MustNotSuspendReason {
    #[primary_span]
    pub span: Span,
    pub reason: String,
}
/* AST_META: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=3 | LINES=20 */

#[derive(Diagnostic)]
#[diag(mir_transform_force_inline)]
#[note]
pub(crate) struct ForceInlineFailure {
    #[label(mir_transform_caller)]
    pub caller_span: Span,
    #[label(mir_transform_callee)]
    pub callee_span: Span,
    #[label(mir_transform_attr)]
    pub attr_span: Span,
    #[primary_span]
    #[label(mir_transform_call)]
    pub call_span: Span,
    pub callee: String,
    pub caller: String,
    pub reason: &'static str,
    #[subdiagnostic]
    pub justification: Option<ForceInlineJustification>,
}
/* AST_META: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Subdiagnostic)]
#[note(mir_transform_force_inline_justification)]
pub(crate) struct ForceInlineJustification {
    pub sym: Symbol,
}