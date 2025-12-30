// Generated macro for impl_66 (impl)
macro_rules! Depcrate_typesimpl_66 {
() => {
// Module: crate::types
// Provides: {"impl_66"}
// Dependencies: {}
impl < T : Marshal > Marshal for Variant < T > { const ALIGN : usize = 1 ; fn signature () -> Cow < 'static , SignatureSingle > { SignatureSingle :: new_unchecked ("v") . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { let sig = T :: signature () ; let s : & Signature = (& * sig) . into () ; s . write_buf (b) ? ; self . 0 . write_buf (b) } }
};
}
