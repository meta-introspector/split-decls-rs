// Generated macro for os_str_to_bstring (function)
macro_rules! Depcrate_envos_str_to_bstring {
() => {
// Module: crate::env
// Provides: {"os_str_to_bstring"}
// Dependencies: {}
# [doc = " Convert the given `input` into a `BString`, useful for usage in `clap`."] pub fn os_str_to_bstring (input : & OsStr) -> Option < BString > { Vec :: from_os_string (input . into ()) . map (Into :: into) . ok () }
};
}
