// Generated macro for Isaac64Core (struct)
macro_rules! Depcrate_isaac64Isaac64Core {
() => {
// Module: crate::isaac64
// Provides: {"Isaac64Core"}
// Dependencies: {}
# [doc = " The core of `Isaac64Rng`, used with `BlockRng`."] # [derive (Clone)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Isaac64Core { # [cfg_attr (feature = "serde" , serde (with = "super::isaac_array::isaac_array_serde"))] mem : [w64 ; RAND_SIZE] , a : w64 , b : w64 , c : w64 , }
};
}
