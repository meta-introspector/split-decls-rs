// Generated macro for Block (struct)
macro_rules! DepcrateBlock {
() => {
// Module: crate
// Provides: {"Block"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct Block { pub header : BlockHeader , pub transactions : Vec < SignedTransaction > , }
};
}
