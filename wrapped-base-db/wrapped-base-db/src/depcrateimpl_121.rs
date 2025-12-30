// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl Drop for DbPanicContext { fn drop (& mut self) { Self :: with_ctx (| ctx | assert ! (ctx . pop () . is_some ())) ; } }
};
}
