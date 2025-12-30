// Generated macro for impl_62 (impl)
macro_rules! Depcrate_typesimpl_62 {
() => {
// Module: crate::types
// Provides: {"impl_62"}
// Dependencies: {}
impl < T1 : Marshal , T2 : Marshal > Marshal for Struct < (T1 , T2) > { const ALIGN : usize = 8 ; fn signature () -> Cow < 'static , SignatureSingle > { let x = format ! ("({}{})" , T1 :: signature () , T2 :: signature ()) ; SignatureSingle :: new_unchecked_owned (x) . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_single (b . align_buf (8)) ? ; (self . 0) . 0 . write_buf (b) ? ; (self . 0) . 1 . write_buf (b) } }
};
}
