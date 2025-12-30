// Generated macro for impl_111 (impl)
macro_rules! Depcrate_recursiveimpl_111 {
() => {
// Module: crate::recursive
// Provides: {"impl_111"}
// Dependencies: {}
impl LintcheckServer { pub fn spawn (options : RecursiveOptions) -> Self { let listener = TcpListener :: bind ("localhost:0") . unwrap () ; let local_addr = listener . local_addr () . unwrap () ; let (sender , receiver) = crossbeam_channel :: unbounded :: < ClippyWarning > () ; let sender = Arc :: new (sender) ; let sender_weak = Arc :: downgrade (& sender) ; let seen = Mutex :: default () ; thread :: spawn (move | | { thread :: scope (| s | { s . spawn (| | { while let Ok ((stream , _)) = listener . accept () { let sender = sender_weak . upgrade () . expect ("received connection after server closed") ; let options = & options ; let seen = & seen ; s . spawn (move | | process_stream (stream , & sender , options , seen)) ; } }) ; }) ; }) ; Self { local_addr , receiver , sender , } } pub fn warnings (self) -> impl Iterator < Item = ClippyWarning > { drop (self . sender) ; self . receiver . into_iter () } }
};
}
