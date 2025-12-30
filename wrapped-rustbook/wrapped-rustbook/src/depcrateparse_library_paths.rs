// Generated macro for parse_library_paths (function)
macro_rules! Depcrateparse_library_paths {
() => {
// Module: crate
// Provides: {"parse_library_paths"}
// Dependencies: {}
fn parse_library_paths (input : & str) -> Result < Vec < String > , String > { Ok (input . split (",") . map (String :: from) . collect ()) }
};
}
