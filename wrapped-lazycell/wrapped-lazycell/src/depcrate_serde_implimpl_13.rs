// Generated macro for impl_13 (impl)
macro_rules! Depcrate_serde_implimpl_13 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for LazyCell < T > { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { deserializer . deserialize_option (LazyCellVisitor (PhantomData)) } }
};
}
