// Generated macro for input_and_compile_error (function)
macro_rules! Depcrateinput_and_compile_error {
() => {
// Module: crate
// Provides: {"input_and_compile_error"}
// Dependencies: {}
# [doc = " Converts the error to a token stream and appends it to the original input."] # [doc = ""] # [doc = " Returning the original input in addition to the error is good for IDEs which can gracefully"] # [doc = " recover and show more precise errors within the macro body."] # [doc = ""] # [doc = " See <https://github.com/rust-analyzer/rust-analyzer/issues/10468> for more info."] fn input_and_compile_error (mut item : TokenStream , err : syn :: Error) -> TokenStream { let compile_err = TokenStream :: from (err . to_compile_error ()) ; item . extend (compile_err) ; item }
};
}
