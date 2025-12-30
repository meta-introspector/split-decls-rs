// Generated macro for impl_233 (impl)
macro_rules! Depcrate_positionimpl_233 {
() => {
// Module: crate::position
// Provides: {"impl_233"}
// Dependencies: {}
impl Hash for Position < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { (self . input as * const str) . hash (state) ; self . pos . hash (state) ; } }
};
}
