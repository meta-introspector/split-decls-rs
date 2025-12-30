// Generated macro for for_each (function)
macro_rules! Depcrate_configfor_each {
() => {
// Module: crate::config
// Provides: {"for_each"}
// Dependencies: {}
fn for_each < I , F , T > (i : I , f : F) where I : Iterator < Item = T > , F : Fn (T) + Sync , T : Send , { # [cfg (windows)] windows_threading :: for_each (i , f) ; # [cfg (not (windows))] for item in i { f (item) ; } }
};
}
