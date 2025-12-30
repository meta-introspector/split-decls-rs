// Generated macro for impl_142 (impl)
macro_rules! Depcrate_latchimpl_142 {
() => {
// Module: crate::latch
// Provides: {"impl_142"}
// Dependencies: {}
impl < L : Latch > Latch for LatchRef < '_ , L > { # [inline] unsafe fn set (this : * const Self) { unsafe { L :: set ((* this) . inner) ; } } }
};
}
