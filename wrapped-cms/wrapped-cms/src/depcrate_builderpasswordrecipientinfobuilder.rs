// Generated macro for PasswordRecipientInfoBuilder (struct)
macro_rules! Depcrate_builderPasswordRecipientInfoBuilder {
() => {
// Module: crate::builder
// Provides: {"PasswordRecipientInfoBuilder"}
// Dependencies: {}
# [doc = " Builds a `PasswordRecipientInfo` according to RFC 5652 § 6 and RFC 3211."] # [doc = " Uses a password or shared secret value to encrypt the content-encryption key."] pub struct PasswordRecipientInfoBuilder < P , R : ? Sized > where P : PwriEncryptor , { # [doc = " Identifies the key-derivation algorithm, and any associated parameters, used to derive the"] # [doc = " key-encryption key from the password or shared secret value. If this field is `None`,"] # [doc = " the key-encryption key is supplied from an external source, for example a hardware crypto"] # [doc = " token such as a smart card."] pub key_derivation_alg : Option < AlgorithmIdentifierOwned > , # [doc = " Encryption algorithm to be used for key encryption"] pub key_enc_alg : AlgorithmIdentifierOwned , # [doc = " Provided password encryptor"] pub key_encryptor : P , # [doc = " Random number generator"] _rng : PhantomData < R > , }
};
}
