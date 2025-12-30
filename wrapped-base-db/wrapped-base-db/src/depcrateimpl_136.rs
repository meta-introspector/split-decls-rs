// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl Drop for DbPanicContext { fn drop (& mut self) { Self :: with_ctx (| ctx | assert ! (ctx . pop () . is_some ())) ; } }
};
}
