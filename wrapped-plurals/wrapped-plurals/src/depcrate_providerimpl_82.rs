// Generated macro for impl_82 (impl)
macro_rules! Depcrate_providerimpl_82 {
() => {
// Module: crate::provider
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl < V > serde :: Serialize for PluralElementsPackedULE < V > where V : PartialEq + serde :: Serialize + VarULE + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { let plural_elements : PluralElementsInner < (FourBitMetadata , & V) > = PluralElementsInner :: from_packed (self) ; plural_elements . serialize (serializer) } else { serializer . serialize_bytes (self . as_bytes ()) } } }
};
}
