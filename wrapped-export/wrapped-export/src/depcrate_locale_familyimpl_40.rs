// Generated macro for impl_40 (impl)
macro_rules! Depcrate_locale_familyimpl_40 {
() => {
// Module: crate::locale_family
// Provides: {"impl_40"}
// Dependencies: {}
impl DataLocaleFamily { # [doc = " Parses a [`DataLocaleFamily`] from a UTF-8 slice."] pub fn try_from_utf8 (code_units : & [u8]) -> Result < Self , DataLocaleFamilyParseError > { if code_units == b"full" { return Ok (Self :: FULL) ; } let (annotation , mut locale) = code_units . split_first () . ok_or (DataLocaleFamilyParseError :: InvalidFamily) ? ; let annotations = match annotation { b'^' => DataLocaleFamilyAnnotations :: without_descendants () , b'%' => DataLocaleFamilyAnnotations :: without_ancestors () , b'@' => DataLocaleFamilyAnnotations :: single () , _ => { locale = code_units ; DataLocaleFamilyAnnotations :: with_descendants () } } ; Ok (Self { locale : Some (DataLocale :: try_from_utf8 (locale) ?) , annotations , }) } # [inline] # [doc = " Parses a [`DataLocaleFamily`]."] pub fn try_from_str (s : & str) -> Result < Self , DataLocaleFamilyParseError > { Self :: try_from_utf8 (s . as_bytes ()) } }
};
}
