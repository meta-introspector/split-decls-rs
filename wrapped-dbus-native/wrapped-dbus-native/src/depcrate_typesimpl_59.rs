// Generated macro for impl_59 (impl)
macro_rules! Depcrate_typesimpl_59 {
() => {
// Module: crate::types
// Provides: {"impl_59"}
// Dependencies: {}
impl Marshal for u8 { const ALIGN : usize = 1 ; fn signature () -> Cow < 'static , SignatureSingle > { SignatureSingle :: new_unchecked ("y") . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_fixed (1 , & [* self]) } }
};
}
