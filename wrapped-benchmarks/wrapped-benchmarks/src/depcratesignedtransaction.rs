// Generated macro for SignedTransaction (struct)
macro_rules! DepcrateSignedTransaction {
() => {
// Module: crate
// Provides: {"SignedTransaction"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct SignedTransaction { transaction : Transaction , signature : Signature , hash : CryptoHash , }
};
}
