// Generated macro for impl_48 (impl)
macro_rules! Depcrate_serdeimpl_48 {
() => {
// Module: crate::serde
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'de , T , S , E > IntoDeserializer < 'de , E > for IndexSet < T , S > where T : IntoDeserializer < 'de , E > + Eq + Hash , S : BuildHasher , E : Error , { type Deserializer = SeqDeserializer < < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { SeqDeserializer :: new (self . into_iter ()) } }
};
}
