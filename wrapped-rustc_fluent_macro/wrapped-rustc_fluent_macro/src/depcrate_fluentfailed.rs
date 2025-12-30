// Generated macro for failed (function)
macro_rules! Depcrate_fluentfailed {
() => {
// Module: crate::fluent
// Provides: {"failed"}
// Dependencies: {}
# [doc = " Tokens to be returned when the macro cannot proceed."] fn failed (crate_name : & Ident) -> proc_macro :: TokenStream { finish (quote ! { pub mod # crate_name { } } , quote ! { "" }) }
};
}
