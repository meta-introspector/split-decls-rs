// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < B : BitBlock > hash :: Hash for BitVec < B > { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . ensure_invariant () ; self . nbits . hash (state) ; for elem in self . blocks () { elem . hash (state) ; } } }
};
}
