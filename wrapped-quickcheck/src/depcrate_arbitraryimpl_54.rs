// Generated macro for impl_54 (impl)
macro_rules! Depcrate_arbitraryimpl_54 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_54"}
// Dependencies: {}
impl Arbitrary for PathBuf { fn arbitrary (g : & mut Gen) -> PathBuf { let here = env :: current_dir () . unwrap_or_else (| _ | PathBuf :: from ("/test/directory")) ; let temp = env :: temp_dir () ; # [allow (deprecated)] let home = env :: home_dir () . unwrap_or_else (| | PathBuf :: from ("/home/user")) ; let mut p = g . choose (& [here , temp , home , PathBuf :: from (".") , PathBuf :: from ("..") , PathBuf :: from ("../../..") , PathBuf :: new () ,]) . unwrap () . to_owned () ; p . extend (Vec :: < OsString > :: arbitrary (g) . iter ()) ; p } fn shrink (& self) -> Box < dyn Iterator < Item = PathBuf > > { let mut shrunk = vec ! [] ; let mut popped = self . clone () ; if popped . pop () { shrunk . push (popped) ; } let normalized = self . iter () . collect :: < PathBuf > () ; if normalized . as_os_str () != self . as_os_str () { shrunk . push (normalized) ; } if let Ok (canonicalized) = self . canonicalize () { if canonicalized . as_os_str () != self . as_os_str () { shrunk . push (canonicalized) ; } } Box :: new (shrunk . into_iter ()) } }
};
}
