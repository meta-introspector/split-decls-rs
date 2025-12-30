// Generated macro for UpgradeableLoaderState (enum)
macro_rules! Depcrate_stateUpgradeableLoaderState {
() => {
// Module: crate::state
// Provides: {"UpgradeableLoaderState"}
// Dependencies: {}
# [doc = " Upgradeable loader account states"] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum UpgradeableLoaderState { # [doc = " Account is not initialized."] Uninitialized , # [doc = " A Buffer account."] Buffer { # [doc = " Authority address"] authority_address : Option < Pubkey > , } , # [doc = " An Program account."] Program { # [doc = " Address of the ProgramData account."] programdata_address : Pubkey , } , ProgramData { # [doc = " Slot that the program was last modified."] slot : u64 , # [doc = " Address of the Program's upgrade authority."] upgrade_authority_address : Option < Pubkey > , } , }
};
}
