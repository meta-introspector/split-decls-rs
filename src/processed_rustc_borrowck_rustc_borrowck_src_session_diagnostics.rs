// SRC: ../rust/compiler/rustc_borrowck/src/session_diagnostics.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::MultiSpan;
use crate::rustc_complete::codes::*;
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::ty::{GenericArg, Ty};
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */
use crate::rustc_complete::Span;

use crate::diagnostics::RegionName;

#[derive(Diagnostic)]
#[diag(borrowck_move_unsized, code = E0161)]
pub(crate) struct MoveUnsized<'tcx> {
    pub ty: Ty<'tcx>,
    #[primary_span]
    #[label]
    pub span: Span,
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(borrowck_higher_ranked_lifetime_error)]
pub(crate) struct HigherRankedLifetimeError {
    #[subdiagnostic]
    pub cause: Option<HigherRankedErrorCause>,
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=8 */

#[derive(Subdiagnostic)]
pub(crate) enum HigherRankedErrorCause {
    #[note(borrowck_could_not_prove)]
    CouldNotProve { predicate: String },
    #[note(borrowck_could_not_normalize)]
    CouldNotNormalize { value: String },
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(borrowck_higher_ranked_subtype_error)]
pub(crate) struct HigherRankedSubtypeError {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Diagnostic)]
#[diag(borrowck_generic_does_not_live_long_enough)]
pub(crate) struct GenericDoesNotLiveLongEnough {
    pub kind: String,
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(LintDiagnostic)]
#[diag(borrowck_var_does_not_need_mut)]
pub(crate) struct VarNeedNotMut {
    #[suggestion(style = "short", applicability = "machine-applicable", code = "")]
    pub span: Span,
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */
#[derive(Diagnostic)]
#[diag(borrowck_var_cannot_escape_closure)]
#[note]
#[note(borrowck_cannot_escape)]
pub(crate) struct FnMutError {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub ty_err: FnMutReturnTypeErr,
}
/* AST_META: AST_ID=10 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=19 */

#[derive(Subdiagnostic)]
pub(crate) enum VarHereDenote {
    #[label(borrowck_var_here_captured)]
    Captured {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_var_here_defined)]
    Defined {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_closure_inferred_mut)]
    FnMutInferred {
        #[primary_span]
        span: Span,
    },
}
/* AST_META: AST_ID=11 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=19 */

#[derive(Subdiagnostic)]
pub(crate) enum FnMutReturnTypeErr {
    #[label(borrowck_returned_closure_escaped)]
    ReturnClosure {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_returned_async_block_escaped)]
    ReturnAsyncBlock {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_returned_ref_escaped)]
    ReturnRef {
        #[primary_span]
        span: Span,
    },
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(borrowck_lifetime_constraints_error)]
pub(crate) struct LifetimeOutliveErr {
    #[primary_span]
    pub span: Span,
}
/* AST_META: AST_ID=13 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=20 */

#[derive(Subdiagnostic)]
pub(crate) enum LifetimeReturnCategoryErr<'a> {
    #[label(borrowck_returned_lifetime_wrong)]
    WrongReturn {
        #[primary_span]
        span: Span,
        mir_def_name: &'a str,
        outlived_fr_name: RegionName,
        fr_name: &'a RegionName,
    },
    #[label(borrowck_returned_lifetime_short)]
    ShortReturn {
        #[primary_span]
        span: Span,
        category_desc: &'static str,
        free_region_name: &'a RegionName,
        outlived_fr_name: RegionName,
    },
}
/* AST_META: AST_ID=14 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=9 */

#[derive(Subdiagnostic)]
pub(crate) enum RequireStaticErr {
    #[note(borrowck_used_impl_require_static)]
    UsedImpl {
        #[primary_span]
        multi_span: MultiSpan,
    },
}
/* AST_META: AST_ID=15 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=12 | LINES=44 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureVarPathUseCause {
    #[label(borrowck_borrow_due_to_use_coroutine)]
    BorrowInCoroutine {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_use_due_to_use_coroutine)]
    UseInCoroutine {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_due_to_use_coroutine)]
    AssignInCoroutine {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_part_due_to_use_coroutine)]
    AssignPartInCoroutine {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_borrow_due_to_use_closure)]
    BorrowInClosure {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_use_due_to_use_closure)]
    UseInClosure {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_due_to_use_closure)]
    AssignInClosure {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_part_due_to_use_closure)]
    AssignPartInClosure {
        #[primary_span]
        path_span: Span,
    },
}
/* AST_META: AST_ID=16 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=19 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureVarKind {
    #[label(borrowck_capture_immute)]
    Immut {
        #[primary_span]
        kind_span: Span,
    },
    #[label(borrowck_capture_mut)]
    Mut {
        #[primary_span]
        kind_span: Span,
    },
    #[label(borrowck_capture_move)]
    Move {
        #[primary_span]
        kind_span: Span,
    },
}
/* AST_META: AST_ID=17 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=19 | LINES=80 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureVarCause {
    #[label(borrowck_var_borrow_by_use_place_in_coroutine)]
    BorrowUsePlaceCoroutine {
        is_single_var: bool,
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_borrow_by_use_place_in_closure)]
    BorrowUsePlaceClosure {
        is_single_var: bool,
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_borrow_by_use_in_coroutine)]
    BorrowUseInCoroutine {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_borrow_by_use_in_closure)]
    BorrowUseInClosure {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_move_by_use_in_coroutine)]
    MoveUseInCoroutine {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_move_by_use_in_closure)]
    MoveUseInClosure {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_first_borrow_by_use_place_in_coroutine)]
    FirstBorrowUsePlaceCoroutine {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_first_borrow_by_use_place_in_closure)]
    FirstBorrowUsePlaceClosure {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_second_borrow_by_use_place_in_coroutine)]
    SecondBorrowUsePlaceCoroutine {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_second_borrow_by_use_place_in_closure)]
    SecondBorrowUsePlaceClosure {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_mutable_borrow_by_use_place_in_closure)]
    MutableBorrowUsePlaceClosure {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_partial_var_move_by_use_in_coroutine)]
    PartialMoveUseInCoroutine {
        #[primary_span]
        var_span: Span,
        is_partial: bool,
    },
    #[label(borrowck_partial_var_move_by_use_in_closure)]
    PartialMoveUseInClosure {
        #[primary_span]
        var_span: Span,
        is_partial: bool,
    },
}
/* AST_META: AST_ID=18 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */

#[derive(Diagnostic)]
#[diag(borrowck_cannot_move_when_borrowed, code = E0505)]
pub(crate) struct MoveBorrow<'a> {
    pub place: &'a str,
    pub borrow_place: &'a str,
    pub value_place: &'a str,
    #[primary_span]
    #[label(borrowck_move_label)]
    pub span: Span,
    #[label]
    pub borrow_span: Span,
}
/* AST_META: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */

#[derive(Diagnostic)]
#[diag(borrowck_opaque_type_lifetime_mismatch)]
pub(crate) struct LifetimeMismatchOpaqueParam<'tcx> {
    pub arg: GenericArg<'tcx>,
    pub prev: GenericArg<'tcx>,
    #[primary_span]
    #[label]
    #[note]
    pub span: Span,
    #[label(borrowck_prev_lifetime_label)]
    pub prev_span: Span,
}
/* AST_META: AST_ID=20 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=11 | LINES=57 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureReasonLabel<'a> {
    #[label(borrowck_moved_due_to_call)]
    Call {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_usage_in_operator)]
    OperatorUse {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_implicit_into_iter_call)]
    ImplicitCall {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_method_call)]
    MethodCall {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_await)]
    Await {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_value_moved_here)]
    MovedHere {
        #[primary_span]
        move_span: Span,
        is_partial: bool,
        is_move_msg: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_consider_borrow_type_contents)]
    BorrowContent {
        #[primary_span]
        var_span: Span,
    },
}
/* AST_META: AST_ID=21 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=26 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureReasonNote {
    #[note(borrowck_moved_a_fn_once_in_call)]
    FnOnceMoveInCall {
        #[primary_span]
        var_span: Span,
    },
    #[note(borrowck_calling_operator_moves)]
    UnOpMoveByOperator {
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_calling_operator_moves_lhs)]
    LhsMoveByOperator {
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_func_take_self_moved_place)]
    FuncTakeSelf {
        func: String,
        place_name: String,
        #[primary_span]
        span: Span,
    },
}
/* AST_META: AST_ID=22 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=25 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureReasonSuggest<'tcx> {
    #[suggestion(
        borrowck_suggest_iterate_over_slice,
        applicability = "maybe-incorrect",
        code = "&",
        style = "verbose"
    )]
    IterateSlice {
        ty: Ty<'tcx>,
        #[primary_span]
        span: Span,
    },
    #[suggestion(
        borrowck_suggest_create_fresh_reborrow,
        applicability = "maybe-incorrect",
        code = ".as_mut()",
        style = "verbose"
    )]
    FreshReborrow {
        #[primary_span]
        span: Span,
    },
}
/* AST_META: AST_ID=23 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=16 */

#[derive(Subdiagnostic)]
pub(crate) enum CaptureArgLabel {
    #[label(borrowck_value_capture_here)]
    Capture {
        is_within: bool,
        #[primary_span]
        args_span: Span,
    },
    #[label(borrowck_move_out_place_here)]
    MoveOutPlace {
        place: String,
        #[primary_span]
        args_span: Span,
    },
}
/* AST_META: AST_ID=24 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=16 */

#[derive(Subdiagnostic)]
pub(crate) enum OnClosureNote<'a> {
    #[note(borrowck_closure_invoked_twice)]
    InvokedTwice {
        place_name: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_closure_moved_twice)]
    MovedTwice {
        place_name: &'a str,
        #[primary_span]
        span: Span,
    },
}
/* AST_META: AST_ID=25 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=14 */

#[derive(Subdiagnostic)]
pub(crate) enum TypeNoCopy<'a, 'tcx> {
    #[label(borrowck_ty_no_impl_copy)]
    Label {
        is_partial_move: bool,
        ty: Ty<'tcx>,
        place: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_ty_no_impl_copy)]
    Note { is_partial_move: bool, ty: Ty<'tcx>, place: &'a str },
}
/* AST_META: AST_ID=26 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(borrowck_simd_intrinsic_arg_const)]
pub(crate) struct SimdIntrinsicArgConst {
    #[primary_span]
    pub span: Span,
    pub arg: usize,
    pub intrinsic: String,
}
/* AST_META: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(LintDiagnostic)]
#[diag(borrowck_tail_expr_drop_order)]
pub(crate) struct TailExprDropOrder {
    #[label]
    pub borrowed: Span,
}