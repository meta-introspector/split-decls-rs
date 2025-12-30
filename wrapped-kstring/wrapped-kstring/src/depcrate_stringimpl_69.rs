// Generated macro for impl_69 (impl)
macro_rules! Depcrate_stringimpl_69 {
() => {
// Module: crate::string
// Provides: {"impl_69"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , B : crate :: backend :: HeapStr > serde :: Deserialize < 'de > for KStringBase < B > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { deserializer . deserialize_string (StringVisitor (std :: marker :: PhantomData)) } }
};
}
