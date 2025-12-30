// Generated macro for BitSetValueTree (struct)
macro_rules! Depcrate_bitsBitSetValueTree {
() => {
// Module: crate::bits
// Provides: {"BitSetValueTree"}
// Dependencies: {}
# [doc = " Value tree produced by `BitSetStrategy` and `SampledBitSetStrategy`."] # [derive (Clone , Copy , Debug)] pub struct BitSetValueTree < T : BitSetLike > { inner : T , shrink : usize , prev_shrink : Option < usize > , min_count : usize , }
};
}
