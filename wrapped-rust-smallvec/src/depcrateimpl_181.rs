// Generated macro for impl_181 (impl)
macro_rules! Depcrateimpl_181 {
() => {
// Module: crate
// Provides: {"impl_181"}
// Dependencies: {}
impl < T : Hash , const N : usize > Hash for SmallVec < T , N > { fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
};
}
