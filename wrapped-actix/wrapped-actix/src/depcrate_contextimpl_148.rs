// Generated macro for impl_148 (impl)
macro_rules! Depcrate_contextimpl_148 {
() => {
// Module: crate::context
// Provides: {"impl_148"}
// Dependencies: {}
impl < A > ActorContext for Context < A > where A : Actor < Context = Self > , { # [inline] fn stop (& mut self) { self . parts . stop () } # [inline] fn terminate (& mut self) { self . parts . terminate () } # [inline] fn state (& self) -> ActorState { self . parts . state () } }
};
}
