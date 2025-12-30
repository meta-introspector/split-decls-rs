// Generated macro for ProgramState (enum)
macro_rules! Depcrate_stateProgramState {
() => {
// Module: crate::state
// Provides: {"ProgramState"}
// Dependencies: {}
# [doc = " Program account states"] # [cfg_attr (feature = "frozen-abi" , derive (AbiEnumVisitor , AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] # [allow (clippy :: large_enum_variant)] pub enum ProgramState { # [doc = " Account is not initialized."] Uninitialized , # [doc = " Initialized `LookupTable` account."] LookupTable (LookupTableMeta) , }
};
}
