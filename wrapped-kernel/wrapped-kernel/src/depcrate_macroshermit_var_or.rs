// Generated macro for hermit_var_or (macro)
macro_rules! Depcrate_macroshermit_var_or {
() => {
// Module: crate::macros
// Provides: {"hermit_var_or"}
// Dependencies: {}
# [doc = " Tries to fetch the specified environment variable with a default value."] # [doc = ""] # [doc = " Fetches according to [`hermit_var`] or returns the specified default value."] # [allow (unused_macros)] macro_rules ! hermit_var_or { ($ name : expr , $ default : expr) => { hermit_var ! ($ name) . as_deref () . unwrap_or ($ default) } ; }
};
}
