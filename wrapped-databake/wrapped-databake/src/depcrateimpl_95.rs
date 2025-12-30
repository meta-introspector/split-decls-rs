// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl CrateEnv { # [doc = " Adds a crate to this collection. This can be called concurrently"] # [doc = " and without `mut`."] pub fn insert (& self , krate : & 'static str) { self . 0 . lock () . expect ("poison") . insert (krate) ; } }
};
}
