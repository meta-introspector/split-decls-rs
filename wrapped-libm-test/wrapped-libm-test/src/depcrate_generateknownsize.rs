// Generated macro for KnownSize (struct)
macro_rules! Depcrate_generateKnownSize {
() => {
// Module: crate::generate
// Provides: {"KnownSize"}
// Dependencies: {}
# [doc = " A wrapper to turn any iterator into an `ExactSizeIterator`. Asserts the final result to ensure"] # [doc = " the provided size was correct."] # [derive (Debug)] pub struct KnownSize < I > { total : u64 , current : u64 , iter : I , }
};
}
