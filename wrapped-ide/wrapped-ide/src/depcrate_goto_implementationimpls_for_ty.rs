// Generated macro for impls_for_ty (function)
macro_rules! Depcrate_goto_implementationimpls_for_ty {
() => {
// Module: crate::goto_implementation
// Provides: {"impls_for_ty"}
// Dependencies: {}
fn impls_for_ty (sema : & Semantics < '_ , RootDatabase > , ty : hir :: Type < '_ >) -> Vec < NavigationTarget > { Impl :: all_for_type (sema . db , ty) . into_iter () . filter_map (| imp | imp . try_to_nav (sema)) . flatten () . collect () }
};
}
