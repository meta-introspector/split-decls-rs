// Generated macro for parse_version_utf8 (function)
macro_rules! Depcrateparse_version_utf8 {
() => {
// Module: crate
// Provides: {"parse_version_utf8"}
// Dependencies: {}
fn parse_version_utf8 (output_bytes : & [u8]) -> Result < Version , VersionError > { let output = str :: from_utf8 (output_bytes) . map_err (| _ | VersionError :: OutputError) ? ; parse_version (output) . map_err (| _ | VersionError :: ParseError (output . to_owned ())) }
};
}
