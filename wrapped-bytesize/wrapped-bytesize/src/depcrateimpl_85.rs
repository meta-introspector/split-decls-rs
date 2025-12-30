// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl < T > ops :: SubAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn sub_assign (& mut self , rhs : T) { self . 0 -= rhs . into () ; } }
};
}
