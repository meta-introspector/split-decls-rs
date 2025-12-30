// Generated macro for impls_for_trait (function)
macro_rules! Depcrate_goto_implementationimpls_for_trait {
() => {
// Module: crate::goto_implementation
// Provides: {"impls_for_trait"}
// Dependencies: {}
fn impls_for_trait (sema : & Semantics < '_ , RootDatabase > , trait_ : hir :: Trait ,) -> Vec < NavigationTarget > { Impl :: all_for_trait (sema . db , trait_) . into_iter () . filter_map (| imp | imp . try_to_nav (sema)) . flatten () . collect () }
};
}
