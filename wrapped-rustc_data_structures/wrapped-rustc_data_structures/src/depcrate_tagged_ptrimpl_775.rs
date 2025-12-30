// Generated macro for impl_775 (impl)
macro_rules! Depcrate_tagged_ptrimpl_775 {
() => {
// Module: crate::tagged_ptr
// Provides: {"impl_775"}
// Dependencies: {}
impl < P , T : Tag > Hash for TaggedRef < '_ , P , T > { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . packed . hash (state) ; } }
};
}
