// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_read_wasmimpl_1078 {
() => {
// Module: crate::read::wasm
// Provides: {"impl_1078"}
// Dependencies: {}
impl < T > ReadError < T > for wasmparser :: Result < T > { fn read_error (self , error : & 'static str) -> Result < T > { self . map_err (| _ | Error (error)) } }
};
}
