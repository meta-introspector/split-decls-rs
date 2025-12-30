// Generated macro for impl_113 (impl)
macro_rules! Depcrate_ext_impls_impl_serdeimpl_113 {
() => {
// Module: crate::ext_impls::impl_serde
// Provides: {"impl_113"}
// Dependencies: {}
impl < 'de , T , N : ArrayLength > Deserialize < 'de > for GenericArray < T , N > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < GenericArray < T , N > , D :: Error > where D : Deserializer < 'de > , { let visitor = GAVisitor { _t : PhantomData , _n : PhantomData , } ; deserializer . deserialize_tuple (N :: USIZE , visitor) } }
};
}
