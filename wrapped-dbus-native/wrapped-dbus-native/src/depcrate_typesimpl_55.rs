// Generated macro for impl_55 (impl)
macro_rules! Depcrate_typesimpl_55 {
() => {
// Module: crate::types
// Provides: {"impl_55"}
// Dependencies: {}
impl Marshal for & Signature { const ALIGN : usize = 1 ; fn signature () -> Cow < 'static , SignatureSingle > { SignatureSingle :: new_unchecked ("g") . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_vectored (& [IoSlice :: new (& [self . len () as u8]) , IoSlice :: new (self . as_bytes ()) , IoSlice :: new (& [0])]) } }
};
}
