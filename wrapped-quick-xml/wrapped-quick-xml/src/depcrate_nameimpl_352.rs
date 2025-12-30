// Generated macro for impl_352 (impl)
macro_rules! Depcrate_nameimpl_352 {
() => {
// Module: crate::name
// Provides: {"impl_352"}
// Dependencies: {}
impl < 'a > Prefix < 'a > { # [doc = " Extracts internal slice"] # [inline (always)] pub const fn into_inner (self) -> & 'a [u8] { self . 0 } # [doc = " Checks if this prefix is a special prefix `xml`."] # [inline (always)] pub const fn is_xml (& self) -> bool { matches ! (self . 0 , b"xml") } # [doc = " Checks if this prefix is a special prefix `xmlns`."] # [inline (always)] pub const fn is_xmlns (& self) -> bool { matches ! (self . 0 , b"xmlns") } }
};
}
