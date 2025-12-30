// Generated macro for _function_name (macro)
macro_rules! Depcrate_macros_function_name {
() => {
// Module: crate::macros
// Provides: {"_function_name"}
// Dependencies: {}
# [doc = " Utility macro to return the name of the current function."] # [doc (hidden)] # [macro_export] macro_rules ! _function_name { () => { { fn f () { } fn type_name_of_val < T > (_ : T) -> &'static str { $ crate :: _macro_support :: any :: type_name ::< T > () } let mut name = type_name_of_val (f) . strip_suffix ("::f") . unwrap_or ("") ; while let Some (rest) = name . strip_suffix ("::{{closure}}") { name = rest ; } name } } ; }
};
}
