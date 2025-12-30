// Generated macro for simple_escapes (function)
macro_rules! Depcrate_cstr_testssimple_escapes {
() => {
// Module: crate::cstr::tests
// Provides: {"simple_escapes"}
// Dependencies: {}
# [test] fn simple_escapes () { check ! (c"a\nb" , true , None) ; check ! (c"\nb" , true , None) ; check ! (c"a\n" , true , None) ; check ! (c"\n" , true , None) ; check ! (c"\x60foo \t bar\rbaz\n banana" , true , None) ; check ! (c"foo \\ferris" , true , None) ; check ! (c"baz \\ferris\"box" , true , None) ; check ! (c"\\foo\\ banana\" baz\"" , true , None) ; check ! (c"\"foo \\ferris \" baz\\" , true , None) ; check ! (c" \x01" , true , None) ; check ! (c"\x0c foo" , true , None) ; check ! (c" foo\x0D " , true , None) ; check ! (c"\\x13" , true , None) ; check ! (c"\"x30" , true , None) ; }
};
}
