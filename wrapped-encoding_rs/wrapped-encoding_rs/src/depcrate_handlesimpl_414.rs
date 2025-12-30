// Generated macro for impl_414 (impl)
macro_rules! Depcrate_handlesimpl_414 {
() => {
// Module: crate::handles
// Provides: {"impl_414"}
// Dependencies: {}
impl < 'a , 'b > Utf16UnreadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new_char (source : & 'a mut Utf16Source < 'b >) -> (char , Self) { let old_pos = source . pos ; let character = source . read () ; (character , Self { source , old_pos }) } # [inline (always)] fn new_enum (source : & 'a mut Utf16Source < 'b >) -> (Unicode , Self) { let old_pos = source . pos ; let character = source . read_enum () ; (character , Self { source , old_pos }) } # [inline (always)] pub fn unread (self) -> usize { self . source . pos = self . old_pos ; self . old_pos } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } # [inline (always)] pub fn commit (self) -> & 'a mut Utf16Source < 'b > { self . source } }
};
}
