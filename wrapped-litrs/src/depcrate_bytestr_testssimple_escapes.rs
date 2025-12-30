// Generated macro for simple_escapes (function)
macro_rules! Depcrate_bytestr_testssimple_escapes {
() => {
// Module: crate::bytestr::tests
// Provides: {"simple_escapes"}
// Dependencies: {}
# [test] fn simple_escapes () { check ! (b"a\nb" , true , None) ; check ! (b"\nb" , true , None) ; check ! (b"a\n" , true , None) ; check ! (b"\n" , true , None) ; check ! (b"\x60foo \t bar\rbaz\n banana \0kiwi" , true , None) ; check ! (b"foo \\ferris" , true , None) ; check ! (b"baz \\ferris\"box" , true , None) ; check ! (b"\\foo\\ banana\" baz\"" , true , None) ; check ! (b"\"foo \\ferris \" baz\\" , true , None) ; check ! (b"\x00" , true , None) ; check ! (b" \x01" , true , None) ; check ! (b"\x0c foo" , true , None) ; check ! (b" foo\x0D " , true , None) ; check ! (b"\\x13" , true , None) ; check ! (b"\"x30" , true , None) ; }
};
}
