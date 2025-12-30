// Generated macro for impl_1277 (impl)
macro_rules! Depcrate_readimpl_1277 {
() => {
// Module: crate::read
// Provides: {"impl_1277"}
// Dependencies: {}
impl < T > ReadError < T > for result :: Result < T , Error > { fn read_error (self , error : & 'static str) -> Result < T > { self . map_err (| _ | Error (error)) } }
};
}
