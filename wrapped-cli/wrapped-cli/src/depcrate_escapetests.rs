// Generated macro for tests (module)
macro_rules! Depcrate_escapetests {
() => {
// Module: crate::escape
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { escape , unescape } ; fn b (bytes : & 'static [u8]) -> Vec < u8 > { bytes . to_vec () } # [test] fn empty () { assert_eq ! (b (b"") , unescape (r"")) ; assert_eq ! (r"" , escape (b"")) ; } # [test] fn backslash () { assert_eq ! (b (b"\\") , unescape (r"\\")) ; assert_eq ! (r"\\" , escape (b"\\")) ; } # [test] fn nul () { assert_eq ! (b (b"\x00") , unescape (r"\x00")) ; assert_eq ! (b (b"\x00") , unescape (r"\0")) ; assert_eq ! (r"\0" , escape (b"\x00")) ; } # [test] fn nl () { assert_eq ! (b (b"\n") , unescape (r"\n")) ; assert_eq ! (r"\n" , escape (b"\n")) ; } # [test] fn tab () { assert_eq ! (b (b"\t") , unescape (r"\t")) ; assert_eq ! (r"\t" , escape (b"\t")) ; } # [test] fn carriage () { assert_eq ! (b (b"\r") , unescape (r"\r")) ; assert_eq ! (r"\r" , escape (b"\r")) ; } # [test] fn nothing_simple () { assert_eq ! (b (b"\\a") , unescape (r"\a")) ; assert_eq ! (b (b"\\a") , unescape (r"\\a")) ; assert_eq ! (r"\\a" , escape (b"\\a")) ; } # [test] fn nothing_hex0 () { assert_eq ! (b (b"\\x") , unescape (r"\x")) ; assert_eq ! (b (b"\\x") , unescape (r"\\x")) ; assert_eq ! (r"\\x" , escape (b"\\x")) ; } # [test] fn nothing_hex1 () { assert_eq ! (b (b"\\xz") , unescape (r"\xz")) ; assert_eq ! (b (b"\\xz") , unescape (r"\\xz")) ; assert_eq ! (r"\\xz" , escape (b"\\xz")) ; } # [test] fn nothing_hex2 () { assert_eq ! (b (b"\\xzz") , unescape (r"\xzz")) ; assert_eq ! (b (b"\\xzz") , unescape (r"\\xzz")) ; assert_eq ! (r"\\xzz" , escape (b"\\xzz")) ; } # [test] fn invalid_utf8 () { assert_eq ! (r"\xFF" , escape (b"\xFF")) ; assert_eq ! (r"a\xFFb" , escape (b"a\xFFb")) ; } }
};
}
