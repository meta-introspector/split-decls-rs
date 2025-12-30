// Generated macro for IsaacArray (struct)
macro_rules! Depcrate_isaac_arrayIsaacArray {
() => {
// Module: crate::isaac_array
// Provides: {"IsaacArray"}
// Dependencies: {}
# [derive (Copy , Clone)] # [allow (missing_debug_implementations)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct IsaacArray < T > { # [cfg_attr (feature = "serde" , serde (with = "isaac_array_serde"))] # [cfg_attr (feature = "serde" , serde (bound (serialize = "T: Serialize" , deserialize = "T: Deserialize<'de> + Copy + Default")))] inner : [T ; RAND_SIZE] , }
};
}
