// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl < T > ops :: MulAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn mul_assign (& mut self , rhs : T) { self . 0 *= rhs . into () ; } }
};
}
