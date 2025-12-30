// Generated macro for format_rust_expression (function)
macro_rules! Depcrate_utilsformat_rust_expression {
() => {
// Module: crate::utils
// Provides: {"format_rust_expression"}
// Dependencies: {}
# [doc = " Tries to format a given rust expression with rustfmt"] pub fn format_rust_expression (value : & str) -> Cow < '_ , str > { const PREFIX : & str = "const x:() = " ; const SUFFIX : & str = ";\n" ; if let Ok (mut proc) = Command :: new ("rustfmt") . arg ("--emit=stdout") . arg ("--edition=2018") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: null ()) . spawn () { { let stdin = proc . stdin . as_mut () . unwrap () ; stdin . write_all (PREFIX . as_bytes ()) . unwrap () ; stdin . write_all (value . as_bytes ()) . unwrap () ; stdin . write_all (SUFFIX . as_bytes ()) . unwrap () ; } if let Ok (output) = proc . wait_with_output () { if output . status . success () { let start = PREFIX . len () + 1 ; let end = output . stdout . len () - SUFFIX . len () ; return std :: str :: from_utf8 (& output . stdout [start .. end]) . unwrap () . replace ("\r\n" , "\n") . into () ; } } } Cow :: Borrowed (value) }
};
}
