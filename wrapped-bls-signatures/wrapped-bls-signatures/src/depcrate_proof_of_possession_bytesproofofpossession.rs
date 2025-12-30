// Generated macro for ProofOfPossession (struct)
macro_rules! Depcrate_proof_of_possession_bytesProofOfPossession {
() => {
// Module: crate::proof_of_possession::bytes
// Provides: {"ProofOfPossession"}
// Dependencies: {}
# [doc = " A serialized BLS proof of possession in an affine point representation."] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Clone , Copy , Debug , Hash , Eq , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct ProofOfPossession (# [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PROOF_OF_POSSESSION_AFFINE_SIZE]"))] pub [u8 ; BLS_PROOF_OF_POSSESSION_AFFINE_SIZE] ,) ;
};
}
