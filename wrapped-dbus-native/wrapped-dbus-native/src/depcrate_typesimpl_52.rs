// Generated macro for impl_52 (impl)
macro_rules! Depcrate_typesimpl_52 {
() => {
// Module: crate::types
// Provides: {"impl_52"}
// Dependencies: {}
impl Marshal for & ObjectPath { const ALIGN : usize = 4 ; fn signature () -> Cow < 'static , SignatureSingle > { SignatureSingle :: new_unchecked ("o") . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_str (& * self) } }
};
}
