// Generated macro for VecValueTree (struct)
macro_rules! Depcrate_collectionVecValueTree {
() => {
// Module: crate::collection
// Provides: {"VecValueTree"}
// Dependencies: {}
# [doc = " `ValueTree` corresponding to `VecStrategy`."] # [derive (Clone , Debug)] pub struct VecValueTree < T : ValueTree > { elements : Vec < T > , included_elements : VarBitSet , min_size : usize , shrink : Shrink , prev_shrink : Option < Shrink > , }
};
}
