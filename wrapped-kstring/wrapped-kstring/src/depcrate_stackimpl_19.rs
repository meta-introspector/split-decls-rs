// Generated macro for impl_19 (impl)
macro_rules! Depcrate_stackimpl_19 {
() => {
// Module: crate::stack
// Provides: {"impl_19"}
// Dependencies: {}
impl < const CAPACITY : usize > std :: hash :: Hash for StackString < CAPACITY > { # [inline] fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_str () . hash (state) ; } }
};
}
