// Generated macro for is_by_value_closure_capture (function)
macro_rules! Depcrate_redundant_localsis_by_value_closure_capture {
() => {
// Module: crate::redundant_locals
// Provides: {"is_by_value_closure_capture"}
// Dependencies: {}
# [doc = " Checks if the enclosing body is a closure and if the given local is captured by value."] # [doc = ""] # [doc = " In those cases, the redefinition may be necessary to force a move:"] # [doc = " ```"] # [doc = " fn assert_static<T: 'static>(_: T) {}"] # [doc = ""] # [doc = " let v = String::new();"] # [doc = " let closure = || {"] # [doc = "   let v = v; // <- removing this redefinition makes `closure` no longer `'static`"] # [doc = "   dbg!(&v);"] # [doc = " };"] # [doc = " assert_static(closure);"] # [doc = " ```"] fn is_by_value_closure_capture (cx : & LateContext < '_ > , redefinition : HirId , root_variable : HirId) -> bool { let closure_def_id = cx . tcx . hir_enclosing_body_owner (redefinition) ; cx . tcx . is_closure_like (closure_def_id . to_def_id ()) && cx . tcx . closure_captures (closure_def_id) . iter () . any (| c | { matches ! (c . info . capture_kind , UpvarCapture :: ByValue) && matches ! (c . place . base , PlaceBase :: Upvar (upvar) if upvar . var_path . hir_id == root_variable) }) }
};
}
