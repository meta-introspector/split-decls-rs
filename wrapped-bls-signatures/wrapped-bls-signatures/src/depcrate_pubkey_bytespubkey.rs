// Generated macro for Pubkey (struct)
macro_rules! Depcrate_pubkey_bytesPubkey {
() => {
// Module: crate::pubkey::bytes
// Provides: {"Pubkey"}
// Dependencies: {}
# [doc = " A serialized BLS public key in an affine point representation."] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Clone , Copy , Debug , Hash , Eq , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct Pubkey (# [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PUBLIC_KEY_AFFINE_SIZE]"))] pub [u8 ; BLS_PUBLIC_KEY_AFFINE_SIZE] ,) ;
};
}
