// Generated macro for impl_14 (impl)
macro_rules! Depcrate_helpersimpl_14 {
() => {
// Module: crate::helpers
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : AsRef < str > , U : AsRef < str > > ToPkgId for (T , U) { fn to_pkgid (& self) -> PackageId { let (name , vers) = self ; PackageId :: try_new (name . as_ref () , vers . as_ref () , registry_loc ()) . unwrap () } }
};
}
