// Generated macro for impl_45 (impl)
macro_rules! Depcrate_backends_linuximpl_45 {
() => {
// Module: crate::backends::linux
// Provides: {"impl_45"}
// Dependencies: {}
impl RawHostInfoBackend for LinuxHostInfoBackend { fn raw_requested_locales () -> Result < Vec < String > , HostInfoError > { if let Ok (s) = std :: env :: var ("LANGUAGE") { let v : Vec < String > = s . split (':') . filter (| x | ! x . is_empty ()) . map (| s | s . to_string ()) . collect () ; if ! v . is_empty () { return Ok (v) ; } } for k in ["LC_MESSAGES" , "LC_ALL" , "LANG"] { if let Ok (s) = std :: env :: var (k) { if ! s . is_empty () { return Ok (vec ! [s]) ; } } } Ok (vec ! []) } }
};
}
