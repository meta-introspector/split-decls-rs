// Generated macro for impl_23 (impl)
macro_rules! Depcrate_hashimpl_23 {
() => {
// Module: crate::hash
// Provides: {"impl_23"}
// Dependencies: {}
impl NumHash for isize { # [inline] fn num_hash < H : Hasher > (& self , state : & mut H) { # [cfg (target_pointer_width = "32")] return (& (* self as i32)) . num_hash (state) ; # [cfg (target_pointer_width = "64")] return (& (* self as i64)) . num_hash (state) ; } }
};
}
