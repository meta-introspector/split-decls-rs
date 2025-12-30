// Generated macro for impl_14 (impl)
macro_rules! Depcrate_serdeimpl_14 {
() => {
// Module: crate::serde
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'de , T , E > IntoDeserializer < 'de , E > for VecList < T > where T : IntoDeserializer < 'de , E > , E : Error , { type Deserializer = SeqDeserializer < < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { SeqDeserializer :: new (self . into_iter ()) } }
};
}
