// Generated macro for LoaderV4Status (enum)
macro_rules! Depcrate_stateLoaderV4Status {
() => {
// Module: crate::state
// Provides: {"LoaderV4Status"}
// Dependencies: {}
# [repr (u64)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum LoaderV4Status { # [doc = " Program is in maintenance"] Retracted , # [doc = " Program is ready to be executed"] Deployed , # [doc = " Same as `Deployed`, but can not be retracted anymore"] Finalized , }
};
}
