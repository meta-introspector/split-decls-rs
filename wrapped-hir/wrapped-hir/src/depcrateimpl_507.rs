// Generated macro for impl_507 (impl)
macro_rules! Depcrateimpl_507 {
() => {
// Module: crate
// Provides: {"impl_507"}
// Dependencies: {}
impl < T : hir_def :: HasModule > HasCrate for T { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () . into () } }
};
}
