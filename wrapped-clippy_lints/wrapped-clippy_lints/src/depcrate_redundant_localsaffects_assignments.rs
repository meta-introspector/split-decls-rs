// Generated macro for affects_assignments (function)
macro_rules! Depcrate_redundant_localsaffects_assignments {
() => {
// Module: crate::redundant_locals
// Provides: {"affects_assignments"}
// Dependencies: {}
# [doc = " Check if a rebinding of a local changes the effect of assignments to the binding."] fn affects_assignments (cx : & LateContext < '_ > , mutability : Mutability , bind : HirId , rebind : HirId) -> bool { mutability == Mutability :: Mut && cx . tcx . hir_get_enclosing_scope (bind) != cx . tcx . hir_get_enclosing_scope (rebind) }
};
}
