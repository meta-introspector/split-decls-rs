// Generated macro for impl_11 (impl)
macro_rules! Depcrate_block_apiimpl_11 {
() => {
// Module: crate::block_api
// Provides: {"impl_11"}
// Dependencies: {}
impl < C : PmacCipher , const LC_SIZE : usize > Drop for PmacState < C , LC_SIZE > { fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . counter . zeroize () ; self . l_inv . zeroize () ; self . l_cache . iter_mut () . for_each (| c | c . zeroize ()) ; self . tag . zeroize () ; self . offset . zeroize () ; } } }
};
}
