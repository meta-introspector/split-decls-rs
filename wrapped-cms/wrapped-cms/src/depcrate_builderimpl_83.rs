// Generated macro for impl_83 (impl)
macro_rules! Depcrate_builderimpl_83 {
() => {
// Module: crate::builder
// Provides: {"impl_83"}
// Dependencies: {}
impl < R > KeyTransRecipientInfoBuilder < R > { # [doc = " Creates a `KeyTransRecipientInfoBuilder`"] pub fn new (rid : RecipientIdentifier , key_encryption_info : KeyEncryptionInfo) -> Result < Self > { Ok (KeyTransRecipientInfoBuilder { rid , key_encryption_info , _rng : PhantomData , }) } }
};
}
