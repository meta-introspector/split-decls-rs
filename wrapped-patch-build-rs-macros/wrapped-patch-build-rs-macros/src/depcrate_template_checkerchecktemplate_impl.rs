// Generated macro for checktemplate_impl (function)
macro_rules! Depcrate_template_checkerchecktemplate_impl {
() => {
// Module: crate::template_checker
// Provides: {"checktemplate_impl"}
// Dependencies: {}
# [decl (fn , name = "checktemplate_impl" , vis = "pub" , hash = "2bad40af")] pub fn checktemplate_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let template_code = input_str . value () ; let parse_result = parse_str :: < syn :: File > (& template_code) ; match parse_result { Ok (_) => { quote ! { { println ! ("cargo:warning=✅ Template syntax check passed") ; # template_code } } . into () } Err (e) => { let error_msg = format ! ("❌ Template syntax error: {}" , e) ; quote ! { { println ! ("cargo:warning={}" , # error_msg) ; compile_error ! (# error_msg) ; } } . into () } } }
};
}
