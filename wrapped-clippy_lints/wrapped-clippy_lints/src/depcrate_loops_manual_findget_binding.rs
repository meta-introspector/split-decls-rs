// Generated macro for get_binding (function)
macro_rules! Depcrate_loops_manual_findget_binding {
() => {
// Module: crate::loops::manual_find
// Provides: {"get_binding"}
// Dependencies: {}
fn get_binding (pat : & Pat < '_ >) -> Option < HirId > { let mut hir_id = None ; let mut count = 0 ; pat . each_binding (| annotation , id , _ , _ | { count += 1 ; if count > 1 { hir_id = None ; return ; } if let BindingMode :: NONE = annotation { hir_id = Some (id) ; } }) ; hir_id }
};
}
