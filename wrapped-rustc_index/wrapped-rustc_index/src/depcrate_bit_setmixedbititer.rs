// Generated macro for MixedBitIter (enum)
macro_rules! Depcrate_bit_setMixedBitIter {
() => {
// Module: crate::bit_set
// Provides: {"MixedBitIter"}
// Dependencies: {}
pub enum MixedBitIter < 'a , T : Idx > { Small (BitIter < 'a , T >) , Large (ChunkedBitIter < 'a , T >) , }
};
}
