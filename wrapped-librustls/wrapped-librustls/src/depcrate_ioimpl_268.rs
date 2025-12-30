// Generated macro for impl_268 (impl)
macro_rules! Depcrate_ioimpl_268 {
() => {
// Module: crate::io
// Provides: {"impl_268"}
// Dependencies: {}
impl Read for CallbackReader { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let mut out_n = 0 ; let cb = self . callback ; let result = unsafe { cb (self . userdata , buf . as_mut_ptr () , buf . len () , & mut out_n) } ; match result . 0 { 0 => Ok (out_n) , e => Err (Error :: from_raw_os_error (e)) , } } }
};
}
