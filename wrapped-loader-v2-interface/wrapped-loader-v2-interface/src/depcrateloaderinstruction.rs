// Generated macro for LoaderInstruction (enum)
macro_rules! DepcrateLoaderInstruction {
() => {
// Module: crate
// Provides: {"LoaderInstruction"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] pub enum LoaderInstruction { # [doc = " Write program data into an account"] # [doc = ""] # [doc = " # Account references"] # [doc = "   0. [WRITE, SIGNER] Account to write to"] Write { # [doc = " Offset at which to write the given bytes"] offset : u32 , # [doc = " Serialized program data"] # [cfg_attr (feature = "serde" , serde (with = "serde_bytes"))] bytes : Vec < u8 > , } , # [doc = " Finalize an account loaded with program data for execution"] # [doc = ""] # [doc = " The exact preparation steps is loader specific but on success the loader must set the executable"] # [doc = " bit of the account."] # [doc = ""] # [doc = " # Account references"] # [doc = "   0. [WRITE, SIGNER] The account to prepare for execution"] # [doc = "   1. [] Rent sysvar"] Finalize , }
};
}
