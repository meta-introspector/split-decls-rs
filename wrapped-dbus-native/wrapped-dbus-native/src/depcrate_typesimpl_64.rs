// Generated macro for impl_64 (impl)
macro_rules! Depcrate_typesimpl_64 {
() => {
// Module: crate::types
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , T : Marshal > Marshal for Array < & 'a [T] > { const ALIGN : usize = 4 ; fn signature () -> Cow < 'static , SignatureSingle > { let x = format ! ("a{}" , T :: signature ()) ; SignatureSingle :: new_unchecked_owned (x) . into () } fn write_buf < B : Write + Seek > (& self , b : & mut MarshalState < B >) -> IoResult < () > { b . write_array (T :: ALIGN , | b | { for elem in self . 0 { elem . write_buf (b) ? ; } Ok (()) }) } }
};
}
