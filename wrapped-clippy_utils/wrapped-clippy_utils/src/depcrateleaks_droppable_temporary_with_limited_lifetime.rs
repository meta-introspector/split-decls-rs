// Generated macro for leaks_droppable_temporary_with_limited_lifetime (function)
macro_rules! Depcrateleaks_droppable_temporary_with_limited_lifetime {
() => {
// Module: crate
// Provides: {"leaks_droppable_temporary_with_limited_lifetime"}
// Dependencies: {}
# [doc = " Returns true if `expr` creates any temporary whose type references a non-static lifetime and has"] # [doc = " a significant drop and does not consume it."] pub fn leaks_droppable_temporary_with_limited_lifetime < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { for_each_unconsumed_temporary (cx , expr , | temporary_ty | { if temporary_ty . has_significant_drop (cx . tcx , cx . typing_env ()) && temporary_ty . walk () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Lifetime (re) if ! re . is_static ())) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_break () }
};
}
