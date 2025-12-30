// Generated macro for impl_146 (impl)
macro_rules! Depcrateimpl_146 {
() => {
// Module: crate
// Provides: {"impl_146"}
// Dependencies: {}
impl PosixTimeZone < Abbreviation > { fn quote (& self) -> proc_macro2 :: TokenStream { let PosixTimeZone { ref std_abbrev , ref std_offset , ref dst } = * self ; let std_abbrev = std_abbrev . as_str () ; let std_offset = std_offset . quote () ; let dst = dst . as_ref () . map (| dst | { let dst = dst . quote () ; quote ! (Some (# dst)) }) . unwrap_or_else (| | quote ! (None)) ; quote ! { jiff :: shared :: PosixTimeZone { std_abbrev : # std_abbrev , std_offset : # std_offset , dst : # dst , } } } }
};
}
