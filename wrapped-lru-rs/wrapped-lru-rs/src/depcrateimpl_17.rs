// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < K : Hash > Hash for KeyRef < K > { fn hash < H : Hasher > (& self , state : & mut H) { unsafe { (* self . k) . hash (state) } } }
};
}
