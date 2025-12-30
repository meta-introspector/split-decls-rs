// Generated macro for BlockHeader (struct)
macro_rules! DepcrateBlockHeader {
() => {
// Module: crate
// Provides: {"BlockHeader"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct BlockHeader { pub inner : BlockHeaderInner , pub signature : Signature , pub hash : CryptoHash , }
};
}
