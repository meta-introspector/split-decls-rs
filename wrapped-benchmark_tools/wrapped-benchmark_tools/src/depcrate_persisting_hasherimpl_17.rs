// Generated macro for impl_17 (impl)
macro_rules! Depcrate_persisting_hasherimpl_17 {
() => {
// Module: crate::persisting_hasher
// Provides: {"impl_17"}
// Dependencies: {}
impl PersistingHasher { fn add_to_hash (& mut self , i : u64) { self . hash = self . hash . rotate_right (31) . wrapping_add (i) . wrapping_mul (0xcfee444d8b59a89b) ; } }
};
}
