// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , solana_keypair :: keypair_from_seed } ; # [test] fn test_presigner () { let keypair = keypair_from_seed (& [0u8 ; 32]) . unwrap () ; let pubkey = keypair . pubkey () ; let data = [1u8] ; let sig = keypair . sign_message (& data) ; let presigner = Presigner :: new (& pubkey , & sig) ; assert_eq ! (presigner . try_pubkey () . unwrap () , pubkey) ; assert_eq ! (presigner . pubkey () , pubkey) ; assert_eq ! (presigner . try_sign_message (& data) . unwrap () , sig) ; assert_eq ! (presigner . sign_message (& data) , sig) ; let bad_data = [2u8] ; assert ! (presigner . try_sign_message (& bad_data) . is_err ()) ; assert_eq ! (presigner . sign_message (& bad_data) , Signature :: default ()) ; assert_eq ! (presigner , keypair) ; assert_eq ! (keypair , presigner) ; let presigner2 = Presigner :: new (& pubkey , & sig) ; assert_eq ! (presigner , presigner2) ; } }
};
}
