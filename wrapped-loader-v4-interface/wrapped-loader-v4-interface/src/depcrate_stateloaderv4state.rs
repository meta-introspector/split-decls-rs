// Generated macro for LoaderV4State (struct)
macro_rules! Depcrate_stateLoaderV4State {
() => {
// Module: crate::state
// Provides: {"LoaderV4State"}
// Dependencies: {}
# [doc = " LoaderV4 account states"] # [repr (C)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub struct LoaderV4State { # [doc = " Slot in which the program was last deployed, retracted or initialized."] pub slot : u64 , # [doc = " Address of signer which can send program management instructions when the status is not finalized."] # [doc = " Otherwise a forwarding to the next version of the finalized program."] pub authority_address_or_next_version : Pubkey , # [doc = " Deployment status."] pub status : LoaderV4Status , }
};
}
