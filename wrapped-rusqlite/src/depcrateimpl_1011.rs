// Generated macro for impl_1011 (impl)
macro_rules! Depcrateimpl_1011 {
() => {
// Module: crate
// Provides: {"impl_1011"}
// Dependencies: {}
impl Drop for Connection { # [inline] fn drop (& mut self) { # [cfg (feature = "cache")] self . flush_prepared_statement_cache () ; } }
};
}
