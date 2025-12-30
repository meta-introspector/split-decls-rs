// Generated macro for impl_22 (impl)
macro_rules! Depcrate_hashimpl_22 {
() => {
// Module: crate::hash
// Provides: {"impl_22"}
// Dependencies: {}
impl NumHash for usize { # [inline] fn num_hash < H : Hasher > (& self , state : & mut H) { # [cfg (target_pointer_width = "32")] return (& (* self as u32)) . num_hash (state) ; # [cfg (target_pointer_width = "64")] return (& (* self as u64)) . num_hash (state) ; } }
};
}
