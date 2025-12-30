// Generated macro for SignatureCompressed (struct)
macro_rules! Depcrate_signature_bytesSignatureCompressed {
() => {
// Module: crate::signature::bytes
// Provides: {"SignatureCompressed"}
// Dependencies: {}
# [doc = " A serialized BLS signature in a compressed point representation"] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Clone , Copy , Debug , Hash , Eq , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct SignatureCompressed (# [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_SIGNATURE_COMPRESSED_SIZE]"))] pub [u8 ; BLS_SIGNATURE_COMPRESSED_SIZE] ,) ;
};
}
