// Generated macro for SlotsIter (struct)
macro_rules! Depcrate_dfa_onepassSlotsIter {
() => {
// Module: crate::dfa::onepass
// Provides: {"SlotsIter"}
// Dependencies: {}
# [doc = " An iterator over all of the bits set in a slot set."] # [doc = ""] # [doc = " This returns the bit index that is set, so callers may need to offset it"] # [doc = " to get the actual NFA slot index."] # [derive (Debug)] struct SlotsIter { slots : Slots , }
};
}
