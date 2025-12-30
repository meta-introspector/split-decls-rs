// Generated macro for impl_39 (impl)
macro_rules! Depcrate_eventimpl_39 {
() => {
// Module: crate::event
// Provides: {"impl_39"}
// Dependencies: {}
impl Hash for Event { fn hash < H : Hasher > (& self , state : & mut H) { self . kind . hash (state) ; self . paths . hash (state) ; self . tracker () . hash (state) ; self . flag () . hash (state) ; self . info () . hash (state) ; self . source () . hash (state) ; } }
};
}
