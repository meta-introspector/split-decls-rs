// Generated macro for impl_57 (impl)
macro_rules! Depcrate_typesimpl_57 {
() => {
// Module: crate::types
// Provides: {"impl_57"}
// Dependencies: {}
impl Marshal for u32 { const ALIGN : usize = 4 ; fn signature () -> Cow < 'static , SignatureSingle > { SignatureSingle :: new_unchecked ("u") . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_fixed (4 , & self . to_ne_bytes ()) } }
};
}
