// Generated macro for impl_353 (impl)
macro_rules! Depcrate_rsliceimpl_353 {
() => {
// Module: crate::rslice
// Provides: {"impl_353"}
// Dependencies: {}
impl fmt :: Debug for rustls_str < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let raw = unsafe { slice :: from_raw_parts (self . data as * const u8 , self . len) } ; let s = str :: from_utf8 (raw) . unwrap_or ("%!(ERROR)") ; f . debug_struct ("rustls_str") . field ("data" , & s) . field ("len" , & self . len) . finish () } }
};
}
