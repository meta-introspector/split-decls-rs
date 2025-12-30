// Generated macro for impl_15 (impl)
macro_rules! Depcrate_argsimpl_15 {
() => {
// Module: crate::args
// Provides: {"impl_15"}
// Dependencies: {}
impl Args for ServerArgs { fn with_docopt (docopt : & docopt :: Docopt) -> Self { let args = docopt . parse () . unwrap_or_else (| e | e . exit ()) ; let listen = args . get_str ("--listen") . to_string () ; let no_retry = args . get_bool ("--no-retry") ; let root = args . get_str ("--root") . to_string () ; let index = args . get_str ("--index") . to_string () ; let cert = args . get_str ("--cert") . to_string () ; let key = args . get_str ("--key") . to_string () ; let disable_gso = args . get_bool ("--disable-gso") ; let disable_pacing = args . get_bool ("--disable-pacing") ; let enable_pmtud = args . get_bool ("--enable-pmtud") ; ServerArgs { listen , no_retry , root , index , cert , key , disable_gso , disable_pacing , enable_pmtud , } } }
};
}
