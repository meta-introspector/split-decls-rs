// Generated macro for impl_157 (impl)
macro_rules! Depcrate_tagsimpl_157 {
() => {
// Module: crate::tags
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'de , T : serde :: de :: Deserialize < 'de > > serde :: de :: Deserialize < 'de > for Tagged < T > { fn deserialize < D : serde :: de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { deserializer . deserialize_any (MaybeTaggedVisitor :: < T > (PhantomData)) } }
};
}
