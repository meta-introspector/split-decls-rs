// Generated macro for assert_unchecked (function)
macro_rules! Depcrate_utilsassert_unchecked {
() => {
// Module: crate::utils
// Provides: {"assert_unchecked"}
// Dependencies: {}
# [allow (dead_code)] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub (crate) unsafe fn assert_unchecked (cond : bool) { if ! cond { if cfg ! (debug_assertions) { unreachable ! () } else { unsafe { core :: hint :: unreachable_unchecked () } } } }
};
}
