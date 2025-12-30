// Generated macro for Signature (struct)
macro_rules! Depcrate_signature_bytesSignature {
() => {
// Module: crate::signature::bytes
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " A serialized BLS signature in an affine point representation"] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Clone , Copy , Debug , Hash , Eq , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct Signature (# [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_SIGNATURE_AFFINE_SIZE]"))] pub [u8 ; BLS_SIGNATURE_AFFINE_SIZE] ,) ;
};
}
