// Generated macro for impl_49 (impl)
macro_rules! Depcrate_typesimpl_49 {
() => {
// Module: crate::types
// Provides: {"impl_49"}
// Dependencies: {}
impl Marshal for & Str { const ALIGN : usize = 4 ; fn signature () -> Cow < 'static , SignatureSingle > { SignatureSingle :: new_unchecked ("s") . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_str (self) } }
};
}
