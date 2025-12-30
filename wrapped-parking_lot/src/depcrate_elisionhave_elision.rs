// Generated macro for have_elision (function)
macro_rules! Depcrate_elisionhave_elision {
() => {
// Module: crate::elision
// Provides: {"have_elision"}
// Dependencies: {}
# [inline] pub fn have_elision () -> bool { cfg ! (all (feature = "hardware-lock-elision" , not (miri) , any (target_arch = "x86" , target_arch = "x86_64") ,)) }
};
}
