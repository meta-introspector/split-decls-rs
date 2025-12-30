// Generated macro for McfHasher (trait)
macro_rules! Depcrate_traitsMcfHasher {
() => {
// Module: crate::traits
// Provides: {"McfHasher"}
// Dependencies: {}
# [doc = " Trait for password hashing algorithms which support the legacy"] # [doc = " [Modular Crypt Format (MCF)][MCF]."] # [doc = ""] # [doc = " [MCF]: https://passlib.readthedocs.io/en/stable/modular_crypt_format.html"] pub trait McfHasher { # [doc = " Upgrade an MCF hash to a PHC hash. MCF follow this rough format:"] # [doc = ""] # [doc = " ```text"] # [doc = " $<id>$<content>"] # [doc = " ```"] # [doc = ""] # [doc = " MCF hashes are otherwise largely unstructured and parsed according to"] # [doc = " algorithm-specific rules so hashers must parse a raw string themselves."] fn upgrade_mcf_hash < 'a > (& self , hash : & 'a str) -> Result < PasswordHash < 'a > > ; # [doc = " Verify a password hash in MCF format against the provided password."] fn verify_mcf_hash (& self , password : & [u8] , mcf_hash : & str) -> Result < () > where Self : PasswordVerifier , { self . verify_password (password , & self . upgrade_mcf_hash (mcf_hash) ?) } }
};
}
