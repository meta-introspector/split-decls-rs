// Generated macro for Account (struct)
macro_rules! DepcrateAccount {
() => {
// Module: crate
// Provides: {"Account"}
// Dependencies: {}
# [doc = " An Account with data that is stored on chain"] # [repr (C)] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample) , frozen_abi (digest = "62EqVoynUFvuui7DVfqWCvZP7bxKGJGioeSBnWrdjRME"))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize) , serde (rename_all = "camelCase"))] # [derive (PartialEq , Eq , Clone , Default)] pub struct Account { # [doc = " lamports in the account"] pub lamports : u64 , # [doc = " data held in this account"] # [cfg_attr (feature = "serde" , serde (with = "serde_bytes"))] pub data : Vec < u8 > , # [doc = " the program that owns this account. If executable, the program that loads this account."] pub owner : Pubkey , # [doc = " this account's data contains a loaded program (and is now read-only)"] pub executable : bool , # [doc = " the epoch at which this account will next owe rent"] pub rent_epoch : Epoch , }
};
}
