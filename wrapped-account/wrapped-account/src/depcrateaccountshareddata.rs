// Generated macro for AccountSharedData (struct)
macro_rules! DepcrateAccountSharedData {
() => {
// Module: crate
// Provides: {"AccountSharedData"}
// Dependencies: {}
# [doc = " An Account with data that is stored on chain"] # [doc = " This will be the in-memory representation of the 'Account' struct data."] # [doc = " The existing 'Account' structure cannot easily change due to downstream projects."] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize) , serde (from = "Account"))] # [derive (PartialEq , Eq , Clone , Default)] pub struct AccountSharedData { # [doc = " lamports in the account"] lamports : u64 , # [doc = " data held in this account"] data : Arc < Vec < u8 > > , # [doc = " the program that owns this account. If executable, the program that loads this account."] owner : Pubkey , # [doc = " this account's data contains a loaded program (and is now read-only)"] executable : bool , # [doc = " the epoch at which this account will next owe rent"] rent_epoch : Epoch , }
};
}
