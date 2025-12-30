// Generated macro for AtomicEpoch (struct)
macro_rules! Depcrate_epochAtomicEpoch {
() => {
// Module: crate::epoch
// Provides: {"AtomicEpoch"}
// Dependencies: {}
# [doc = " An atomic value that holds an `Epoch`."] # [derive (Default , Debug)] pub (crate) struct AtomicEpoch { # [doc = " Since `Epoch` is just a wrapper around `usize`, an `AtomicEpoch` is similarly represented"] # [doc = " using an `AtomicUsize`."] data : AtomicUsize , }
};
}
