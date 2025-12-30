// Generated macro for impl_121 (impl)
macro_rules! Depcrate_manifest_rust_versionimpl_121 {
() => {
// Module: crate::manifest::rust_version
// Provides: {"impl_121"}
// Dependencies: {}
impl RustVersion { pub fn is_compatible_with (& self , rustc : & PartialVersion) -> bool { let msrv = self . 0 . to_caret_req () ; let rustc = semver :: Version { major : rustc . major , minor : rustc . minor . unwrap_or_default () , patch : rustc . patch . unwrap_or_default () , pre : Default :: default () , build : Default :: default () , } ; msrv . matches (& rustc) } pub fn into_partial (self) -> PartialVersion { self . 0 } pub fn as_partial (& self) -> & PartialVersion { & self . 0 } }
};
}
