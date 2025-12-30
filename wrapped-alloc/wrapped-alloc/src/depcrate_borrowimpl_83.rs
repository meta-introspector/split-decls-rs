// Generated macro for impl_83 (impl)
macro_rules! Depcrate_borrowimpl_83 {
() => {
// Module: crate::borrow
// Provides: {"impl_83"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : ? Sized > Hash for Cow < '_ , B > where B : Hash + ToOwned , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { Hash :: hash (& * * self , state) } }
};
}
