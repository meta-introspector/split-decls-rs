// Generated macro for PubkeyCompressed (struct)
macro_rules! Depcrate_pubkey_bytesPubkeyCompressed {
() => {
// Module: crate::pubkey::bytes
// Provides: {"PubkeyCompressed"}
// Dependencies: {}
# [doc = " A serialized BLS public key in a compressed point representation."] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Clone , Copy , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct PubkeyCompressed (# [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PUBLIC_KEY_COMPRESSED_SIZE]"))] pub [u8 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] ,) ;
};
}
