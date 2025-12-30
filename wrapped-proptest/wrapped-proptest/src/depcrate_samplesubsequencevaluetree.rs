// Generated macro for SubsequenceValueTree (struct)
macro_rules! Depcrate_sampleSubsequenceValueTree {
() => {
// Module: crate::sample
// Provides: {"SubsequenceValueTree"}
// Dependencies: {}
# [doc = " `ValueTree` type for `Subsequence`."] # [derive (Debug , Clone)] pub struct SubsequenceValueTree < T : Clone + 'static > { values : Arc < Cow < 'static , [T] > > , inner : BitSetValueTree < VarBitSet > , }
};
}
