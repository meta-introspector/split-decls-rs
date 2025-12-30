// Generated macro for Transaction (struct)
macro_rules! DepcrateTransaction {
() => {
// Module: crate
// Provides: {"Transaction"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct Transaction { signer_id : AccountId , public_key : PublicKey , nonce : Nonce , receiver_id : AccountId , actions : Vec < Action > , }
};
}
