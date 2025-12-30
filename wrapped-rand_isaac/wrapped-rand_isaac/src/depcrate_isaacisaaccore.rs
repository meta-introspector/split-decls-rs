// Generated macro for IsaacCore (struct)
macro_rules! Depcrate_isaacIsaacCore {
() => {
// Module: crate::isaac
// Provides: {"IsaacCore"}
// Dependencies: {}
# [doc = " The core of [`IsaacRng`], used with [`BlockRng`]."] # [derive (Clone)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct IsaacCore { # [cfg_attr (feature = "serde" , serde (with = "super::isaac_array::isaac_array_serde"))] mem : [w32 ; RAND_SIZE] , a : w32 , b : w32 , c : w32 , }
};
}
