// Generated macro for is_dyn_sym (function)
macro_rules! Depcrate_shims_unix_foreign_itemsis_dyn_sym {
() => {
// Module: crate::shims::unix::foreign_items
// Provides: {"is_dyn_sym"}
// Dependencies: {}
pub fn is_dyn_sym (name : & str , target_os : & str) -> bool { match name { "isatty" => true , "signal" => true , "getentropy" | "getrandom" => true , _ => match target_os { "android" => android :: is_dyn_sym (name) , "freebsd" => freebsd :: is_dyn_sym (name) , "linux" => linux :: is_dyn_sym (name) , "macos" => macos :: is_dyn_sym (name) , "solaris" | "illumos" => solarish :: is_dyn_sym (name) , _ => false , } , } }
};
}
