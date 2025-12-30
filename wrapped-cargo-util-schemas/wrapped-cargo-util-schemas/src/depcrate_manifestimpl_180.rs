// Generated macro for impl_180 (impl)
macro_rules! Depcrate_manifestimpl_180 {
() => {
// Module: crate::manifest
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for ProfilePackageSpec { fn deserialize < D > (d : D) -> Result < ProfilePackageSpec , D :: Error > where D : de :: Deserializer < 'de > , { let string = String :: deserialize (d) ? ; if string == "*" { Ok (ProfilePackageSpec :: All) } else { PackageIdSpec :: parse (& string) . map_err (de :: Error :: custom) . map (ProfilePackageSpec :: Spec) } } }
};
}
