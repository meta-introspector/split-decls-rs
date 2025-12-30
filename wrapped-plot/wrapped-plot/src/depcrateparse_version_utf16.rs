// Generated macro for parse_version_utf16 (function)
macro_rules! Depcrateparse_version_utf16 {
() => {
// Module: crate
// Provides: {"parse_version_utf16"}
// Dependencies: {}
fn parse_version_utf16 (output_bytes : & [u8]) -> Result < Version , VersionError > { if output_bytes . len () % 2 != 0 { return Err (VersionError :: OutputError) ; } let output_as_u16 : Vec < u16 > = output_bytes . chunks_exact (2) . map (| chunk | u16 :: from_le_bytes ([chunk [0] , chunk [1]])) . collect () ; let output = String :: from_utf16 (& output_as_u16) . map_err (| _ | VersionError :: OutputError) ? ; parse_version (& output) . map_err (| _ | VersionError :: ParseError (output . to_owned ())) }
};
}
