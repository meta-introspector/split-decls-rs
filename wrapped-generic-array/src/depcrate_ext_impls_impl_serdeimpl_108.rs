// Generated macro for impl_108 (impl)
macro_rules! Depcrate_ext_impls_impl_serdeimpl_108 {
() => {
// Module: crate::ext_impls::impl_serde
// Provides: {"impl_108"}
// Dependencies: {}
impl < T , N : ArrayLength > Serialize for GenericArray < T , N > where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut tup = serializer . serialize_tuple (N :: USIZE) ? ; for el in self { tup . serialize_element (el) ? ; } tup . end () } }
};
}
