// Generated macro for impl_47 (impl)
macro_rules! Depcrate_serdeimpl_47 {
() => {
// Module: crate::serde
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de , T , S > Deserialize < 'de > for IndexSet < T , S > where T : Deserialize < 'de > + Eq + Hash , S : Default + BuildHasher , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (IndexSetVisitor (PhantomData)) } }
};
}
