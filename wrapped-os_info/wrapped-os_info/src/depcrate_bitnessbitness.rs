// Generated macro for Bitness (enum)
macro_rules! Depcrate_bitnessBitness {
() => {
// Module: crate::bitness
// Provides: {"Bitness"}
// Dependencies: {}
# [doc = " Operating system architecture in terms of how many bits compose the basic values it can deal with."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [non_exhaustive] pub enum Bitness { # [doc = " Unknown bitness (unable to determine)."] Unknown , # [doc = " 32-bit."] X32 , # [doc = " 64-bit."] X64 , }
};
}
