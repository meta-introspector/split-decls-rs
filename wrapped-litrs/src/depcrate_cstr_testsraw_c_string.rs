// Generated macro for raw_c_string (function)
macro_rules! Depcrate_cstr_testsraw_c_string {
() => {
// Module: crate::cstr::tests
// Provides: {"raw_c_string"}
// Dependencies: {}
# [test] fn raw_c_string () { check ! (cr"" , false , Some (0)) ; check ! (cr"a" , false , Some (0)) ; check ! (cr"peter" , false , Some (0)) ; check ! (cr"Greetings jason!" , false , Some (0)) ; check ! (cr#""# , false , Some (1)) ; check ! (cr#"a"# , false , Some (1)) ; check ! (cr##"peter"## , false , Some (2)) ; check ! (cr###"Greetings # Jason!"### , false , Some (3)) ; check ! (cr########"we ## need #### more ####### hashtags"######## , false , Some (8)) ; check ! (cr#"foo " bar"# , false , Some (1)) ; check ! (cr##"foo " bar"## , false , Some (2)) ; check ! (cr#"foo """" '"'" bar"# , false , Some (1)) ; check ! (cr#""foo""# , false , Some (1)) ; check ! (cr###""foo'"### , false , Some (3)) ; check ! (cr#""x'#_#s'"# , false , Some (1)) ; check ! (cr"#" , false , Some (0)) ; check ! (cr"foo#" , false , Some (0)) ; check ! (cr"##bar" , false , Some (0)) ; check ! (cr###""##foo"##bar'"### , false , Some (3)) ; check ! (cr"foo\n\t\r\0\\x60\u{123}doggo" , false , Some (0)) ; check ! (cr#"cat\n\t\r\0\\x60\u{123}doggo"# , false , Some (1)) ; }
};
}
