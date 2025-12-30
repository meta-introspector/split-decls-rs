// Generated macro for tests (module)
macro_rules! Depcrate_keypairtests {
() => {
// Module: crate::keypair
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , tempfile :: NamedTempFile } ; # [test] fn test_keygen_derive () { let ikm = b"test_ikm" ; let secret = SecretKey :: derive (ikm) . unwrap () ; let public : Pubkey = PubkeyProjective :: from_secret (& secret) . into () ; let keypair = Keypair :: derive (ikm) . unwrap () ; assert_eq ! (keypair . secret , secret) ; assert_eq ! (keypair . public , public) ; } # [test] # [cfg (feature = "solana-signer-derive")] fn test_keygen_derive_from_signer () { let solana_keypair = solana_keypair :: Keypair :: new () ; let secret = SecretKey :: derive_from_signer (& solana_keypair , b"alpenglow-vote") . unwrap () ; let public : Pubkey = PubkeyProjective :: from_secret (& secret) . into () ; let keypair = Keypair :: derive_from_signer (& solana_keypair , b"alpenglow-vote") . unwrap () ; assert_eq ! (keypair . secret , secret) ; assert_eq ! (keypair . public , public) ; } # [test] # [cfg (feature = "std")] fn test_keypair_file () { let temp_keypair_file = NamedTempFile :: new () . unwrap () ; let original_keypair = Keypair :: new () ; original_keypair . write_json_file (& temp_keypair_file) . unwrap () ; let read_keypair = Keypair :: read_json_file (& temp_keypair_file) . unwrap () ; assert_eq ! (original_keypair , read_keypair) ; } }
};
}
