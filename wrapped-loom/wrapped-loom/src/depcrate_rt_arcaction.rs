// Generated macro for Action (enum)
macro_rules! Depcrate_rt_arcAction {
() => {
// Module: crate::rt::arc
// Provides: {"Action"}
// Dependencies: {}
# [doc = " Actions performed on the Arc"] # [doc = ""] # [doc = " Clones are only dependent with inspections. Drops are dependent between each"] # [doc = " other."] # [derive (Debug , Copy , Clone , PartialEq)] pub (super) enum Action { # [doc = " Clone the arc"] RefInc , # [doc = " Drop the Arc"] RefDec , # [doc = " Inspect internals (such as get ref count). This is done with SeqCst"] # [doc = " causality"] Inspect , }
};
}
