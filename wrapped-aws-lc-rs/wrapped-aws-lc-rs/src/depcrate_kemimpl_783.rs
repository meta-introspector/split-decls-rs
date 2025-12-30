// Generated macro for impl_783 (impl)
macro_rules! Depcrate_kemimpl_783 {
() => {
// Module: crate::kem
// Provides: {"impl_783"}
// Dependencies: {}
impl < Id > Algorithm < Id > where Id : AlgorithmIdentifier , { # [doc = " Returns the identifier for this algorithm."] # [must_use] pub fn id (& self) -> Id { self . id } # [inline] # [allow (dead_code)] pub (crate) fn decapsulate_key_size (& self) -> usize { self . decapsulate_key_size } # [inline] pub (crate) fn encapsulate_key_size (& self) -> usize { self . encapsulate_key_size } # [inline] pub (crate) fn ciphertext_size (& self) -> usize { self . ciphertext_size } # [inline] pub (crate) fn shared_secret_size (& self) -> usize { self . shared_secret_size } }
};
}
