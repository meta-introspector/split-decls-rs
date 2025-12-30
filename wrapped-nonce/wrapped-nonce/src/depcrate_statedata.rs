// Generated macro for Data (struct)
macro_rules! Depcrate_stateData {
() => {
// Module: crate::state
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Initialized data of a durable transaction nonce account."] # [doc = ""] # [doc = " This is stored within [`State`] for initialized nonce accounts."] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , Default , PartialEq , Eq , Clone)] pub struct Data { # [doc = " Address of the account that signs transactions using the nonce account."] pub authority : Pubkey , # [doc = " Durable nonce value derived from a valid previous blockhash."] pub durable_nonce : DurableNonce , # [doc = " The fee calculator associated with the blockhash."] pub fee_calculator : FeeCalculator , }
};
}
