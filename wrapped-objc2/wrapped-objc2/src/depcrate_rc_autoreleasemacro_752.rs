// Generated macro for macro_752 (macro)
macro_rules! Depcrate_rc_autoreleasemacro_752 {
() => {
// Module: crate::rc::autorelease
// Provides: {"macro_752"}
// Dependencies: {}
# [cfg (all (debug_assertions , not (feature = "unstable-autoreleasesafe")))] thread_local ! { # [doc = " We track the thread's pools to verify that object lifetimes are only"] # [doc = " taken from the innermost pool."] static POOLS : RefCell < Vec <* mut c_void >> = const { RefCell :: new (Vec :: new ()) } ; }
};
}
