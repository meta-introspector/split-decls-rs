// Generated macro for parent_test_module (function)
macro_rules! Depcrate_runnablesparent_test_module {
() => {
// Module: crate::runnables
// Provides: {"parent_test_module"}
// Dependencies: {}
fn parent_test_module (sema : & Semantics < '_ , RootDatabase > , fn_def : & ast :: Fn) -> Option < hir :: Module > { fn_def . syntax () . ancestors () . find_map (| node | { let module = ast :: Module :: cast (node) ? ; let module = sema . to_def (& module) ? ; if has_test_function_or_multiple_test_submodules (sema , & module , false) { Some (module) } else { None } }) }
};
}
