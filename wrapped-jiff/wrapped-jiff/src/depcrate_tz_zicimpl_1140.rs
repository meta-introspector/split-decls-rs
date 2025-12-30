// Generated macro for impl_1140 (impl)
macro_rules! Depcrate_tz_zicimpl_1140 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1140"}
// Dependencies: {}
impl RuleSaveP { # [doc = " Returns this \"save\" time as an offset."] # [doc = ""] # [doc = " If the span is too long to fit in an offset, then an error is returned."] fn to_offset (& self) -> Result < Offset , Error > { let seconds = Span :: from_invariant_nanoseconds (Unit :: Second , self . span . 0 . to_invariant_nanoseconds () ,) ? . get_seconds () ; let seconds = i32 :: try_from (seconds) . map_err (| _ | { Error :: range ("SAVE seconds" , seconds , i32 :: MIN , i32 :: MAX) }) ? ; Offset :: from_seconds (seconds) } # [doc = " Returns the suffix for this SAVE field."] # [doc = ""] # [doc = " When the suffix is absent, a default is selected based on the time"] # [doc = " span in the field."] fn suffix (& self) -> RuleSaveSuffixP { self . suffix . unwrap_or_else (| | { if self . span . 0 . is_zero () { RuleSaveSuffixP :: Standard } else { RuleSaveSuffixP :: Dst } }) } }
};
}
