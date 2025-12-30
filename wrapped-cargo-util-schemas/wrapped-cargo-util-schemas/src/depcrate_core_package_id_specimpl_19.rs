// Generated macro for impl_19 (impl)
macro_rules! Depcrate_core_package_id_specimpl_19 {
() => {
// Module: crate::core::package_id_spec
// Provides: {"impl_19"}
// Dependencies: {}
impl ser :: Serialize for PackageIdSpec { fn serialize < S > (& self , s : S) -> std :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . to_string () . serialize (s) } }
};
}
