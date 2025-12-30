// Generated macro for impl_750 (impl)
macro_rules! Depcrate_rc_autoreleaseimpl_750 {
() => {
// Module: crate::rc::autorelease
// Provides: {"impl_750"}
// Dependencies: {}
impl Drop for Pool { # [inline] fn drop (& mut self) { # [cfg (all (debug_assertions , not (feature = "unstable-autoreleasesafe")))] POOLS . with (| c | { assert_eq ! (c . borrow_mut () . pop () , Some (self . context) , "popped pool that was not the innermost pool") ; }) ; # [cfg (not (all (target_os = "macos" , target_arch = "x86")))] unsafe { ffi :: objc_autoreleasePoolPop (self . context) ; } } }
};
}
