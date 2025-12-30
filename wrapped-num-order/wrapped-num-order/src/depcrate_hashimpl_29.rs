// Generated macro for impl_29 (impl)
macro_rules! Depcrate_hashimpl_29 {
() => {
// Module: crate::hash
// Provides: {"impl_29"}
// Dependencies: {}
impl NumHash for f64 { fn num_hash < H : Hasher > (& self , state : & mut H) { self . fhash () . num_hash (state) } }
};
}
