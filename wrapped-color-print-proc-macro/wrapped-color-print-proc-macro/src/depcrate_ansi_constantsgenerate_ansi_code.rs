// Generated macro for generate_ansi_code (function)
macro_rules! Depcrate_ansi_constantsgenerate_ansi_code {
() => {
// Module: crate::ansi_constants
// Provides: {"generate_ansi_code"}
// Dependencies: {}
# [doc = " Generate an SGR ANSI sequence."] pub fn generate_ansi_code (params : & [u8]) -> String { let mut ansi_code = String :: from ("\u{1b}[") ; let mut first = true ; for param in params { if first { first = false ; } else { ansi_code . push (';') ; } ansi_code . push_str (& format ! ("{}" , param)) ; } ansi_code . push ('m') ; ansi_code }
};
}
