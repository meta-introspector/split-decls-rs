// Generated macro for RefModify (enum)
macro_rules! Depcrate_rt_arcRefModify {
() => {
// Module: crate::rt::arc
// Provides: {"RefModify"}
// Dependencies: {}
# [doc = " Actions which modify the Arc's reference count"] # [doc = ""] # [doc = " This is used to ascertain dependence for Action::Inspect"] # [derive (Debug , Copy , Clone , PartialEq)] enum RefModify { # [doc = " Corresponds to Action::RefInc"] RefInc , # [doc = " Corresponds to Action::RefDec"] RefDec , }
};
}
