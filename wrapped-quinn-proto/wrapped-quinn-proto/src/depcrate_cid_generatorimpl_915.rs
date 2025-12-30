// Generated macro for impl_915 (impl)
macro_rules! Depcrate_cid_generatorimpl_915 {
() => {
// Module: crate::cid_generator
// Provides: {"impl_915"}
// Dependencies: {}
impl RandomConnectionIdGenerator { # [doc = " Initialize Random CID generator with a fixed CID length"] # [doc = ""] # [doc = " The given length must be less than or equal to MAX_CID_SIZE."] pub fn new (cid_len : usize) -> Self { debug_assert ! (cid_len <= MAX_CID_SIZE) ; Self { cid_len , .. Self :: default () } } # [doc = " Set the lifetime of CIDs created by this generator"] pub fn set_lifetime (& mut self , d : Duration) -> & mut Self { self . lifetime = Some (d) ; self } }
};
}
