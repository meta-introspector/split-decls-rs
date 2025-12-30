// Generated macro for impl_86 (impl)
macro_rules! Depcrate_builderimpl_86 {
() => {
// Module: crate::builder
// Provides: {"impl_86"}
// Dependencies: {}
impl < R > KekRecipientInfoBuilder < R > { # [doc = " Creates a `KekRecipientInfoBuilder`"] pub fn new (kek_id : KekIdentifier , key_enc_alg : AlgorithmIdentifierOwned) -> Result < Self > { Ok (KekRecipientInfoBuilder { kek_id , key_enc_alg , _rng : PhantomData , }) } }
};
}
