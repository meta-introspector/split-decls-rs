// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] pub mod tests { use { super :: * , solana_system_interface :: instruction :: SystemInstruction } ; # [test] fn test_limited_deserialize_advance_nonce_account () { let item = SystemInstruction :: AdvanceNonceAccount ; let mut serialized = bincode :: serialize (& item) . unwrap () ; assert_eq ! (serialized . len () , 4 , "`SanitizedMessage::get_durable_nonce()` may need a change") ; assert_eq ! (limited_deserialize ::< SystemInstruction > (& serialized , 4) . as_ref () , Ok (& item)) ; assert ! (limited_deserialize ::< SystemInstruction > (& serialized , 3) . is_err ()) ; serialized . push (0) ; assert_eq ! (limited_deserialize ::< SystemInstruction > (& serialized , 4) . as_ref () , Ok (& item)) ; } }
};
}
