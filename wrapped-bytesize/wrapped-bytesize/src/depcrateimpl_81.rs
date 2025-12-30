// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > ops :: AddAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn add_assign (& mut self , rhs : T) { self . 0 += rhs . into () ; } }
};
}
