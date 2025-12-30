// Generated macro for ValidatorStake (struct)
macro_rules! DepcrateValidatorStake {
() => {
// Module: crate
// Provides: {"ValidatorStake"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct ValidatorStake { pub account_id : AccountId , pub public_key : PublicKey , pub amount : Balance , }
};
}
