// Generated macro for impl_1278 (impl)
macro_rules! Depcrate_readimpl_1278 {
() => {
// Module: crate::read
// Provides: {"impl_1278"}
// Dependencies: {}
impl < T > ReadError < T > for Option < T > { fn read_error (self , error : & 'static str) -> Result < T > { self . ok_or (Error (error)) } }
};
}
