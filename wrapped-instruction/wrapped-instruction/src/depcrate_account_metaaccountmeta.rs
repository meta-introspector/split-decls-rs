// Generated macro for AccountMeta (struct)
macro_rules! Depcrate_account_metaAccountMeta {
() => {
// Module: crate::account_meta
// Provides: {"AccountMeta"}
// Dependencies: {}
# [doc = " Describes a single account read or written by a program during instruction"] # [doc = " execution."] # [doc = ""] # [doc = " When constructing an [`Instruction`], a list of all accounts that may be"] # [doc = " read or written during the execution of that instruction must be supplied."] # [doc = " Any account that may be mutated by the program during execution, either its"] # [doc = " data or metadata such as held lamports, must be writable."] # [doc = ""] # [doc = " Note that because the Solana runtime schedules parallel transaction"] # [doc = " execution around which accounts are writable, care should be taken that only"] # [doc = " accounts which actually may be mutated are specified as writable. As the"] # [doc = " default [`AccountMeta::new`] constructor creates writable accounts, this is"] # [doc = " a minor hazard: use [`AccountMeta::new_readonly`] to specify that an account"] # [doc = " is not writable."] # [doc = ""] # [doc = " [`Instruction`]: crate::Instruction"] # [repr (C)] # [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] # [derive (Debug , Default , PartialEq , Eq , Clone)] pub struct AccountMeta { # [doc = " An account's public key."] pub pubkey : Pubkey , # [doc = " True if an `Instruction` requires a `Transaction` signature matching `pubkey`."] pub is_signer : bool , # [doc = " True if the account data or metadata may be mutated during program execution."] pub is_writable : bool , }
};
}
