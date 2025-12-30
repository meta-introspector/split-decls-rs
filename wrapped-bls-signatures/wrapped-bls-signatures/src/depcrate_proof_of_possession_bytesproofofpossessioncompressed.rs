// Generated macro for ProofOfPossessionCompressed (struct)
macro_rules! Depcrate_proof_of_possession_bytesProofOfPossessionCompressed {
() => {
// Module: crate::proof_of_possession::bytes
// Provides: {"ProofOfPossessionCompressed"}
// Dependencies: {}
# [doc = " A serialized BLS proof of possession in a compressed point representation."] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Clone , Copy , Debug , Hash , Eq , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct ProofOfPossessionCompressed (# [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE]"))] pub [u8 ; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE] ,) ;
};
}
