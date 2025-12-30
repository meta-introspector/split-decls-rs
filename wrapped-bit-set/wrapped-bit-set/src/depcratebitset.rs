// Generated macro for BitSet (struct)
macro_rules! DepcrateBitSet {
() => {
// Module: crate
// Provides: {"BitSet"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] # [cfg_attr (feature = "borsh" , derive (borsh :: BorshDeserialize , borsh :: BorshSerialize))] # [cfg_attr (feature = "miniserde" , derive (miniserde :: Deserialize , miniserde :: Serialize))] # [cfg_attr (feature = "nanoserde" , derive (DeBin , DeJson , DeRon , SerBin , SerJson , SerRon))] pub struct BitSet < B = u32 > { bit_vec : BitVec < B > , }
};
}
