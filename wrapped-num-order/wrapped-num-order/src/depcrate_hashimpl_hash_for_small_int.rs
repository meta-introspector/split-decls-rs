// Generated macro for impl_hash_for_small_int (macro)
macro_rules! Depcrate_hashimpl_hash_for_small_int {
() => {
// Module: crate::hash
// Provides: {"impl_hash_for_small_int"}
// Dependencies: {}
macro_rules ! impl_hash_for_small_int { ($ ($ signed : ty) *) => ($ (impl NumHash for $ signed { # [inline] fn num_hash < H : Hasher > (& self , state : & mut H) { (& (* self as i128)) . hash (state) } }) *) ; }
};
}
