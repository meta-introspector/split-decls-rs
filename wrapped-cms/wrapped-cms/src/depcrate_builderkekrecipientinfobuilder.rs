// Generated macro for KekRecipientInfoBuilder (struct)
macro_rules! Depcrate_builderKekRecipientInfoBuilder {
() => {
// Module: crate::builder
// Provides: {"KekRecipientInfoBuilder"}
// Dependencies: {}
# [doc = " Builds a `KekRecipientInfo` according to RFC 5652 § 6."] # [doc = " Uses symmetric key-encryption keys: the content-encryption key is"] # [doc = " encrypted in a previously distributed symmetric key-encryption key."] pub struct KekRecipientInfoBuilder < R : ? Sized > { # [doc = " Specifies a symmetric key-encryption key that was previously distributed to the sender and"] # [doc = " one or more recipients."] pub kek_id : KekIdentifier , # [doc = " Encryption algorithm to be used for key encryption"] pub key_enc_alg : AlgorithmIdentifierOwned , _rng : PhantomData < R > , }
};
}
