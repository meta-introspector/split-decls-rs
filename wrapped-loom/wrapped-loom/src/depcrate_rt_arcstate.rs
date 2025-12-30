// Generated macro for State (struct)
macro_rules! Depcrate_rt_arcState {
() => {
// Module: crate::rt::arc
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct State { # [doc = " Reference count"] ref_cnt : usize , # [doc = " Location where the arc was allocated"] allocated : Location , # [doc = " Causality transfers between threads"] # [doc = ""] # [doc = " Only updated on on ref dec and acquired before drop"] synchronize : Synchronize , # [doc = " Tracks access to the arc object"] last_ref_inc : Option < Access > , last_ref_dec : Option < Access > , last_ref_inspect : Option < Access > , last_ref_modification : Option < RefModify > , }
};
}
