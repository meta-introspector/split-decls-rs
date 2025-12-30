// Generated macro for impl_67 (impl)
macro_rules! Depcrate_versionimpl_67 {
() => {
// Module: crate::version
// Provides: {"impl_67"}
// Dependencies: {}
impl Display for Version { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match * self { Self :: Unknown => f . write_str ("Unknown") , Self :: Semantic (major , minor , patch) => write ! (f , "{major}.{minor}.{patch}") , Self :: Rolling (ref date) => { let date = match date { Some (date) => format ! (" ({date})") , None => "" . to_owned () , } ; write ! (f , "Rolling Release{date}") } Self :: Custom (ref version) => write ! (f , "{version}") , } } }
};
}
