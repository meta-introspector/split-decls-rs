// Generated macro for impl_235 (impl)
macro_rules! Depcrate_manifestimpl_235 {
() => {
// Module: crate::manifest
// Provides: {"impl_235"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for StringOrBool { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . bool (| b | Ok (StringOrBool :: Bool (b))) . string (| s | Ok (StringOrBool :: String (s . to_owned ()))) . deserialize (deserializer) } }
};
}
