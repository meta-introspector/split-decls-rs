// Generated macro for impl_20 (impl)
macro_rules! Depcrate_core_package_id_specimpl_20 {
() => {
// Module: crate::core::package_id_spec
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for PackageIdSpec { fn deserialize < D > (d : D) -> std :: result :: Result < PackageIdSpec , D :: Error > where D : de :: Deserializer < 'de > , { let string = String :: deserialize (d) ? ; PackageIdSpec :: parse (& string) . map_err (de :: Error :: custom) } }
};
}
