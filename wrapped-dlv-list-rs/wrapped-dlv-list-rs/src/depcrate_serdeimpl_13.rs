// Generated macro for impl_13 (impl)
macro_rules! Depcrate_serdeimpl_13 {
() => {
// Module: crate::serde
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for VecList < T > { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { deserializer . deserialize_seq (VecListVisitor (PhantomData)) } }
};
}
