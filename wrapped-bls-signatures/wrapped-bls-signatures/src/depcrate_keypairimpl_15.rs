// Generated macro for impl_15 (impl)
macro_rules! Depcrate_keypairimpl_15 {
() => {
// Module: crate::keypair
// Provides: {"impl_15"}
// Dependencies: {}
impl Keypair { # [doc = " Constructs a new, random `Keypair` using `OsRng`"] # [allow (clippy :: new_without_default)] pub fn new () -> Self { let secret = SecretKey :: new () ; let public = PubkeyProjective :: from_secret (& secret) . into () ; Self { secret , public } } # [doc = " Derive a `Keypair` from a seed (input key material)"] pub fn derive (ikm : & [u8]) -> Result < Self , BlsError > { let secret = SecretKey :: derive (ikm) ? ; let public = PubkeyProjective :: from_secret (& secret) . into () ; Ok (Self { secret , public }) } # [doc = " Derive a `BlsSecretKey` from a Solana signer"] # [cfg (feature = "solana-signer-derive")] pub fn derive_from_signer (signer : & dyn Signer , public_seed : & [u8]) -> Result < Self , BlsError > { let secret = SecretKey :: derive_from_signer (signer , public_seed) ? ; let public = PubkeyProjective :: from_secret (& secret) . into () ; Ok (Self { secret , public }) } # [doc = " Generate a proof of possession for the given keypair"] pub fn proof_of_possession (& self , payload : Option < & [u8] >) -> ProofOfPossessionProjective { self . secret . proof_of_possession (payload) } # [doc = " Sign a message using the provided secret key"] pub fn sign (& self , message : & [u8]) -> SignatureProjective { self . secret . sign (message) } # [doc = " Verify a signature against a message and a public key"] pub fn verify < S : AsSignature > (& self , signature : & S , message : & [u8]) -> Result < bool , BlsError > { self . public . verify_signature (signature , message) } }
};
}
