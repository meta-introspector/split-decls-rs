// Generated macro for impl_233 (impl)
macro_rules! Depcrate_manifestimpl_233 {
() => {
// Module: crate::manifest
// Provides: {"impl_233"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for StringOrVec { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("string or list of strings") . string (| value | Ok (StringOrVec (vec ! [value . to_owned ()]))) . seq (| value | value . deserialize () . map (StringOrVec)) . deserialize (deserializer) } }
};
}
