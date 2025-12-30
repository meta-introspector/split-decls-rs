// Generated macro for raw_byte_string (function)
macro_rules! Depcrate_bytestr_testsraw_byte_string {
() => {
// Module: crate::bytestr::tests
// Provides: {"raw_byte_string"}
// Dependencies: {}
# [test] fn raw_byte_string () { check ! (br"" , false , Some (0)) ; check ! (br"a" , false , Some (0)) ; check ! (br"peter" , false , Some (0)) ; check ! (br"Greetings jason!" , false , Some (0)) ; check ! (br#""# , false , Some (1)) ; check ! (br#"a"# , false , Some (1)) ; check ! (br##"peter"## , false , Some (2)) ; check ! (br###"Greetings # Jason!"### , false , Some (3)) ; check ! (br########"we ## need #### more ####### hashtags"######## , false , Some (8)) ; check ! (br#"foo " bar"# , false , Some (1)) ; check ! (br##"foo " bar"## , false , Some (2)) ; check ! (br#"foo """" '"'" bar"# , false , Some (1)) ; check ! (br#""foo""# , false , Some (1)) ; check ! (br###""foo'"### , false , Some (3)) ; check ! (br#""x'#_#s'"# , false , Some (1)) ; check ! (br"#" , false , Some (0)) ; check ! (br"foo#" , false , Some (0)) ; check ! (br"##bar" , false , Some (0)) ; check ! (br###""##foo"##bar'"### , false , Some (3)) ; check ! (br"foo\n\t\r\0\\x60\u{123}doggo" , false , Some (0)) ; check ! (br#"cat\n\t\r\0\\x60\u{123}doggo"# , false , Some (1)) ; }
};
}
