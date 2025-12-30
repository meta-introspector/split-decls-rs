// Generated macro for impl_143 (impl)
macro_rules! Depcrate_manifestimpl_143 {
() => {
// Module: crate::manifest
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InheritableSemverVersion { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("SemVer version") . string (| value | match value . trim () . parse () . map_err (de :: Error :: custom) { Ok (parsed) => Ok (InheritableField :: Value (parsed)) , Err (e) => Err (e) , } ,) . map (| value | value . deserialize () . map (InheritableField :: Inherit)) . deserialize (d) } }
};
}
