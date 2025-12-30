// Generated macro for impl_13 (impl)
macro_rules! Depcrate_helpersimpl_13 {
() => {
// Module: crate::helpers
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > ToPkgId for & 'a str { fn to_pkgid (& self) -> PackageId { PackageId :: try_new (* self , "1.0.0" , registry_loc ()) . unwrap () } }
};
}
