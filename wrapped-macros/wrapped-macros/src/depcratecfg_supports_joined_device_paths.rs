// Generated macro for cfg_supports_joined_device_paths (function)
macro_rules! Depcratecfg_supports_joined_device_paths {
() => {
// Module: crate
// Provides: {"cfg_supports_joined_device_paths"}
// Dependencies: {}
# [proc_macro_attribute] pub fn cfg_supports_joined_device_paths (args : TokenStream , item : TokenStream ,) -> TokenStream { let Ok (enable) = args . to_string () . parse :: < bool > () else { return Error :: new_spanned (args , "argument must be a boolean") . into_compile_error () ; } ; let supported = match get_windows_version () { Ok (version_info) => version_info . dwBuildNumber < 21000 , Err (error) => return error . into_compile_error () , } ; if enable == supported { item } else { TokenStream :: new () } }
};
}
