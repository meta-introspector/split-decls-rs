// Generated macro for impl_239 (impl)
macro_rules! Depcrate_manifestimpl_239 {
() => {
// Module: crate::manifest
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for VecStringOrBool { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("a boolean or vector of strings") . bool (| value | Ok (VecStringOrBool :: Bool (value))) . seq (| value | value . deserialize () . map (VecStringOrBool :: VecString)) . deserialize (deserializer) } }
};
}
