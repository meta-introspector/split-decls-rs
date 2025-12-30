// Generated macro for impl_16 (impl)
macro_rules! Depcrate_serde_implimpl_16 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for AtomicLazyCell < T > { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { deserializer . deserialize_option (AtomicLazyCellVisitor (PhantomData)) } }
};
}
