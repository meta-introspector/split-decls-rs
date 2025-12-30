// Generated macro for KeyTransRecipientInfoBuilder (struct)
macro_rules! Depcrate_builderKeyTransRecipientInfoBuilder {
() => {
// Module: crate::builder
// Provides: {"KeyTransRecipientInfoBuilder"}
// Dependencies: {}
# [doc = " Builds a `KeyTransRecipientInfo` according to RFC 5652 § 6."] # [doc = " This type uses the recipient's public key to encrypt the content-encryption key."] pub struct KeyTransRecipientInfoBuilder < R : ? Sized > { # [doc = " Identifies the recipient"] pub rid : RecipientIdentifier , # [doc = " Info for key encryption"] pub key_encryption_info : KeyEncryptionInfo , _rng : PhantomData < R > , }
};
}
