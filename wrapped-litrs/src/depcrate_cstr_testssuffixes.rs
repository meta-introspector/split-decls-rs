// Generated macro for suffixes (function)
macro_rules! Depcrate_cstr_testssuffixes {
() => {
// Module: crate::cstr::tests
// Provides: {"suffixes"}
// Dependencies: {}
# [test] fn suffixes () { check ! (c"hello" , r###"c"hello"suffix"### , false , None , "suffix") ; check ! (c"fox" , r#"c"fox"peter"# , false , None , "peter") ; check ! (c"a\x0cb\\" , r#"c"a\x0cb\\"_jürgen"# , true , None , "_jürgen") ; check ! (cr"a\x0cb\\" , r###"cr#"a\x0cb\\"#_jürgen"### , false , Some (1) , "_jürgen") ; }
};
}
