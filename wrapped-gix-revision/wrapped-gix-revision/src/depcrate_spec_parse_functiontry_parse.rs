// Generated macro for try_parse (function)
macro_rules! Depcrate_spec_parse_functiontry_parse {
() => {
// Module: crate::spec::parse::function
// Provides: {"try_parse"}
// Dependencies: {}
fn try_parse < T : FromStr + PartialEq + Default > (input : & BStr) -> Result < Option < T > , Error > { input . to_str () . ok () . and_then (| n | { n . parse () . ok () . map (| n | { if n == T :: default () && input [0] == b'-' { return Err (Error :: NegativeZero { input : input . into () }) ; } Ok (n) }) }) . transpose () }
};
}
