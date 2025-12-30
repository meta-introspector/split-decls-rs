// Generated macro for DerefDelegate (struct)
macro_rules! Depcrate_suggDerefDelegate {
() => {
// Module: crate::sugg
// Provides: {"DerefDelegate"}
// Dependencies: {}
# [doc = " Visitor struct used for tracking down"] # [doc = " dereferencing and borrowing of closure's args"] struct DerefDelegate < 'a , 'tcx > { # [doc = " The late context of the lint"] cx : & 'a LateContext < 'tcx > , # [doc = " The span of the input closure to adapt"] closure_span : Span , # [doc = " The `hir_id` of the closure argument being checked"] closure_arg_id : HirId , # [doc = " Indicates if the arg of the closure is a type annotated double reference"] closure_arg_is_type_annotated_double_ref : bool , # [doc = " last position of the span to gradually build the suggestion"] next_pos : BytePos , # [doc = " `hir_id`s that has been checked. This is used to avoid checking the same `hir_id` multiple"] # [doc = " times when inside macro expansions."] checked_borrows : FxHashSet < HirId > , # [doc = " starting part of the gradually built suggestion"] suggestion_start : String , # [doc = " confidence on the built suggestion"] applicability : Applicability , }
};
}
