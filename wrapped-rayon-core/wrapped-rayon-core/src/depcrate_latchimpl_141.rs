// Generated macro for impl_141 (impl)
macro_rules! Depcrate_latchimpl_141 {
() => {
// Module: crate::latch
// Provides: {"impl_141"}
// Dependencies: {}
impl < L > Deref for LatchRef < '_ , L > { type Target = L ; fn deref (& self) -> & L { unsafe { & * self . inner } } }
};
}
