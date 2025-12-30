// Generated macro for impl_412 (impl)
macro_rules! Depcrate_handlesimpl_412 {
() => {
// Module: crate::handles
// Provides: {"impl_412"}
// Dependencies: {}
impl < 'a , 'b > Utf16ReadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (src : & 'a mut Utf16Source < 'b >) -> Utf16ReadHandle < 'a , 'b > { Utf16ReadHandle { source : src } } # [inline (always)] pub fn read (self) -> (char , Utf16UnreadHandle < 'a , 'b >) { Utf16UnreadHandle :: new_char (self . source) } # [inline (always)] pub fn read_enum (self) -> (Unicode , Utf16UnreadHandle < 'a , 'b >) { Utf16UnreadHandle :: new_enum (self . source) } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } }
};
}
