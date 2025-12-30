// Generated macro for module_name_to_str (function)
macro_rules! Depcrate_back_ltomodule_name_to_str {
() => {
// Module: crate::back::lto
// Provides: {"module_name_to_str"}
// Dependencies: {}
fn module_name_to_str (c_str : & CStr) -> & str { c_str . to_str () . unwrap_or_else (| e | { bug ! ("Encountered non-utf8 GCC module name `{}`: {}" , c_str . to_string_lossy () , e) }) }
};
}
