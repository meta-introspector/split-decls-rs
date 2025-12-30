// Generated macro for Account (struct)
macro_rules! DepcrateAccount {
() => {
// Module: crate
// Provides: {"Account"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct Account { pub amount : Balance , pub staked : Balance , pub code_hash : CryptoHash , pub storage_usage : u64 , pub storage_paid_at : u64 , }
};
}
