// Generated macro for FollowEpsilon (enum)
macro_rules! Depcrate_pikevmFollowEpsilon {
() => {
// Module: crate::pikevm
// Provides: {"FollowEpsilon"}
// Dependencies: {}
# [doc = " A representation of an explicit stack frame when following epsilon"] # [doc = " transitions. This is used to avoid recursion."] # [derive (Clone , Debug)] enum FollowEpsilon { # [doc = " Follow transitions at the given instruction pointer."] IP (InstPtr) , # [doc = " Restore the capture slot with the given position in the input."] Capture { slot : usize , pos : Slot } , }
};
}
