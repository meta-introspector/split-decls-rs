// Generated macro for impl_46 (impl)
macro_rules! Depcrate_ustrimpl_46 {
() => {
// Module: crate::ustr
// Provides: {"impl_46"}
// Dependencies: {}
impl PotentialUtf16 { # [doc = " Create a [`PotentialUtf16`] from a u16 slice."] # [inline] pub const fn from_slice (other : & [u16]) -> & Self { unsafe { core :: mem :: transmute (other) } } pub fn chars (& self) -> impl Iterator < Item = char > + '_ { char :: decode_utf16 (self . 0 . iter () . copied ()) . map (| c | c . unwrap_or (char :: REPLACEMENT_CHARACTER)) } }
};
}
