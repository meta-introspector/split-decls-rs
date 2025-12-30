// Generated macro for impl_1324 (impl)
macro_rules! Depcrate_versionimpl_1324 {
() => {
// Module: crate::version
// Provides: {"impl_1324"}
// Dependencies: {}
impl fmt :: Debug for Version { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { let mut f = f . debug_struct ("Version") ; f . field ("major" , & self . major) . field ("minor" , & self . minor) . field ("rev" , & self . rev) . field ("crate_version" , & self . crate_version ()) . field ("vendored" , & self . vendored ()) . field ("threads" , & self . threads ()) . field ("https" , & self . https ()) . field ("ssh" , & self . ssh ()) . field ("nsec" , & self . nsec ()) ; f . finish () } }
};
}
