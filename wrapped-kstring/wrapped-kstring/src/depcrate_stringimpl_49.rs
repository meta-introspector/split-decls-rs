// Generated macro for impl_49 (impl)
macro_rules! Depcrate_stringimpl_49 {
() => {
// Module: crate::string
// Provides: {"impl_49"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: hash :: Hash for KStringBase < B > { # [inline] fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_str () . hash (state) ; } }
};
}
