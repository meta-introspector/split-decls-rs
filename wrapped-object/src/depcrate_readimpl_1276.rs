// Generated macro for impl_1276 (impl)
macro_rules! Depcrate_readimpl_1276 {
() => {
// Module: crate::read
// Provides: {"impl_1276"}
// Dependencies: {}
impl < T > ReadError < T > for result :: Result < T , () > { fn read_error (self , error : & 'static str) -> Result < T > { self . map_err (| () | Error (error)) } }
};
}
