// Generated macro for impl_43 (impl)
macro_rules! Depcrate_serdeimpl_43 {
() => {
// Module: crate::serde
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'de , K , V , S , E > IntoDeserializer < 'de , E > for IndexMap < K , V , S > where K : IntoDeserializer < 'de , E > + Eq + Hash , V : IntoDeserializer < 'de , E > , S : BuildHasher , E : Error , { type Deserializer = MapDeserializer < 'de , < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { MapDeserializer :: new (self . into_iter ()) } }
};
}
