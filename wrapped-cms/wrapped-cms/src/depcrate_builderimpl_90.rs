// Generated macro for impl_90 (impl)
macro_rules! Depcrate_builderimpl_90 {
() => {
// Module: crate::builder
// Provides: {"impl_90"}
// Dependencies: {}
impl < P , R > PasswordRecipientInfoBuilder < P , R > where P : PwriEncryptor , { # [doc = " Creates a `PasswordRecipientInfoBuilder`"] # [doc = " `key_derivation_alg`: (optional) Algorithm used to derive the"] # [doc = "     key-encryption key from the shared secret (password)"] # [doc = " `key_enc_alg`: Algorithm used to (symmetrically) encrypt the"] # [doc = "     content-encryption key"] # [doc = " `key_encryptor`: Provided encryptor, which is used to encrypt"] # [doc = "     the content-encryption key"] # [doc = " `rng`: Random number generator, required for padding values."] pub fn new (key_encryptor : P) -> Result < Self > { Ok (PasswordRecipientInfoBuilder { key_derivation_alg : key_encryptor . key_derivation_algorithm () ? , key_enc_alg : key_encryptor . key_encryption_algorithm () ? , key_encryptor , _rng : PhantomData , }) } }
};
}
