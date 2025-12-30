// Generated macro for HardForks (struct)
macro_rules! DepcrateHardForks {
() => {
// Module: crate
// Provides: {"HardForks"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct HardForks { hard_forks : Vec < (u64 , usize) > , }
};
}
