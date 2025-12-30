// Generated macro for set_current_dir (function)
macro_rules! Depcrate_envset_current_dir {
() => {
// Module: crate::env
// Provides: {"set_current_dir"}
// Dependencies: {}
# [doc = " A wrapper around [`std::env::set_current_dir`] which includes the directory"] # [doc = " path in the panic message."] # [track_caller] pub fn set_current_dir < P : AsRef < std :: path :: Path > > (dir : P) { std :: env :: set_current_dir (dir . as_ref ()) . expect (& format ! ("could not set current directory to \"{}\"" , dir . as_ref () . display ())) ; }
};
}
