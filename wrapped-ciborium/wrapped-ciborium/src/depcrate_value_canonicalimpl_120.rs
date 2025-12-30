// Generated macro for impl_120 (impl)
macro_rules! Depcrate_value_canonicalimpl_120 {
() => {
// Module: crate::value::canonical
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for CanonicalValue { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { Value :: deserialize (deserializer) . map (Into :: into) } fn deserialize_in_place < D > (deserializer : D , place : & mut Self) -> Result < () , D :: Error > where D : de :: Deserializer < 'de > , { Value :: deserialize_in_place (deserializer , & mut place . 0) } }
};
}
