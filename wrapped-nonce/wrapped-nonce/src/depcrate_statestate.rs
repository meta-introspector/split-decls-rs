// Generated macro for State (enum)
macro_rules! Depcrate_stateState {
() => {
// Module: crate::state
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state of a durable transaction nonce account."] # [doc = ""] # [doc = " When created in memory with [`State::default`] or when deserialized from an"] # [doc = " uninitialized account, a nonce account will be [`State::Uninitialized`]."] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , Default , PartialEq , Eq , Clone)] pub enum State { # [default] Uninitialized , Initialized (Data) , }
};
}
