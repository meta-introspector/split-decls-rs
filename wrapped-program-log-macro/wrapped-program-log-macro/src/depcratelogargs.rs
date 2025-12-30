// Generated macro for LogArgs (struct)
macro_rules! DepcrateLogArgs {
() => {
// Module: crate
// Provides: {"LogArgs"}
// Dependencies: {}
# [doc = " Represents the input arguments to the `log!` macro."] struct LogArgs { # [doc = " The length of the buffer to use for the logger."] # [doc = ""] # [doc = " This does not have effect when the literal `str` does"] # [doc = " not have value placeholders."] buffer_len : LitInt , # [doc = " The literal formatting string passed to the macro."] # [doc = ""] # [doc = " The `str` might have value placeholders. While this is"] # [doc = " not a requirement, the number of placeholders must"] # [doc = " match the number of args."] format_string : LitStr , # [doc = " The arguments passed to the macro."] # [doc = ""] # [doc = " The arguments represent the values to replace the"] # [doc = " placeholders on the format `str`. Valid values must implement"] # [doc = " the [`Log`] trait."] args : Punctuated < Expr , Token ! [,] > , }
};
}
