// Generated macro for encode_rustflags (function)
macro_rules! Depcrateencode_rustflags {
() => {
// Module: crate
// Provides: {"encode_rustflags"}
// Dependencies: {}
# [doc = " Encode a list of rustflags for use in CARGO_ENCODED_RUSTFLAGS."] pub fn encode_rustflags (flags : & [OsString]) -> OsString { let mut res = OsString :: new () ; for flag in flags { if ! res . is_empty () { res . push (OsStr :: new ("\x1f")) ; } let flag = flag . to_str () . expect ("rustflags must be valid UTF-8") ; if flag . contains ('\x1f') { panic ! ("rustflags must not contain `\\x1f` separator") ; } res . push (flag) ; } res }
};
}
