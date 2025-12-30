// Generated macro for StartStateIter (struct)
macro_rules! Depcrate_dfa_denseStartStateIter {
() => {
// Module: crate::dfa::dense
// Provides: {"StartStateIter"}
// Dependencies: {}
# [doc = " An iterator over start state IDs."] # [doc = ""] # [doc = " This iterator yields a triple of start state ID, the anchored mode and the"] # [doc = " start state type. If a pattern ID is relevant, then the anchored mode will"] # [doc = " contain it. Start states with an anchored mode containing a pattern ID will"] # [doc = " only occur when the DFA was compiled with start states for each pattern"] # [doc = " (which is disabled by default)."] pub (crate) struct StartStateIter < 'a > { st : StartTable < & 'a [u32] > , i : usize , }
};
}
