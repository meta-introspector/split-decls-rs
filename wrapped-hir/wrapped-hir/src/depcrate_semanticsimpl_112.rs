// Generated macro for impl_112 (impl)
macro_rules! Depcrate_semanticsimpl_112 {
() => {
// Module: crate::semantics
// Provides: {"impl_112"}
// Dependencies: {}
impl Semantics < '_ , dyn HirDatabase > { # [doc = " Creates an instance that's weakly coupled to its underlying database type."] pub fn new_dyn (db : & '_ dyn HirDatabase) -> Semantics < '_ , dyn HirDatabase > { let impl_ = SemanticsImpl :: new (db) ; Semantics { db , imp : impl_ } } }
};
}
