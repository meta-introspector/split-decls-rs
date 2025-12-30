// Generated macro for BlockHeaderInner (struct)
macro_rules! DepcrateBlockHeaderInner {
() => {
// Module: crate
// Provides: {"BlockHeaderInner"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct BlockHeaderInner { pub height : BlockIndex , pub epoch_hash : CryptoHash , pub prev_hash : CryptoHash , pub prev_state_root : MerkleHash , pub tx_root : MerkleHash , pub timestamp : u64 , pub approval_mask : Vec < bool > , pub approval_sigs : Vec < Signature > , pub total_weight : Weight , pub validator_proposals : Vec < ValidatorStake > , }
};
}
