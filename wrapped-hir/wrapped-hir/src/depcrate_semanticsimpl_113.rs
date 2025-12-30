// Generated macro for impl_113 (impl)
macro_rules! Depcrate_semanticsimpl_113 {
() => {
// Module: crate::semantics
// Provides: {"impl_113"}
// Dependencies: {}
impl < DB : HirDatabase > Semantics < '_ , DB > { # [doc = " Creates an instance that's strongly coupled to its underlying database type."] pub fn new (db : & DB) -> Semantics < '_ , DB > { let impl_ = SemanticsImpl :: new (db) ; Semantics { db , imp : impl_ } } }
};
}
