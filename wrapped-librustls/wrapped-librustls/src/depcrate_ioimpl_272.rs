// Generated macro for impl_272 (impl)
macro_rules! Depcrate_ioimpl_272 {
() => {
// Module: crate::io
// Provides: {"impl_272"}
// Dependencies: {}
impl Write for CallbackWriter { fn write (& mut self , buf : & [u8]) -> Result < usize > { let mut out_n = 0 ; let cb = self . callback ; let result = unsafe { cb (self . userdata , buf . as_ptr () , buf . len () , & mut out_n) } ; match result . 0 { 0 => Ok (out_n) , e => Err (Error :: from_raw_os_error (e)) , } } fn flush (& mut self) -> Result < () > { Ok (()) } }
};
}
