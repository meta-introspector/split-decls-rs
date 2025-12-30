// Generated macro for impl_1434 (impl)
macro_rules! Depcrate_stringimpl_1434 {
() => {
// Module: crate::string
// Provides: {"impl_1434"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl Clone for String { # [track_caller] fn clone (& self) -> Self { String { vec : self . vec . clone () } } # [doc = " Clones the contents of `source` into `self`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible."] # [track_caller] fn clone_from (& mut self , source : & Self) { self . vec . clone_from (& source . vec) ; } }
};
}
