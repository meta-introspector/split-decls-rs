// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
impl < T : Primitive > fmt :: Debug for AtomicMaybeUninit < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (core :: any :: type_name :: < Self > ()) } }
};
}
