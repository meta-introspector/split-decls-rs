// Generated macro for suffixes (function)
macro_rules! Depcrate_bytestr_testssuffixes {
() => {
// Module: crate::bytestr::tests
// Provides: {"suffixes"}
// Dependencies: {}
# [test] fn suffixes () { check ! (b"hello" , r###"b"hello"suffix"### , false , None , "suffix") ; check ! (b"fox" , r#"b"fox"peter"# , false , None , "peter") ; check ! (b"a\x0cb\\" , r#"b"a\x0cb\\"_jürgen"# , true , None , "_jürgen") ; check ! (br"a\x0cb\\" , r###"br#"a\x0cb\\"#_jürgen"### , false , Some (1) , "_jürgen") ; }
};
}
