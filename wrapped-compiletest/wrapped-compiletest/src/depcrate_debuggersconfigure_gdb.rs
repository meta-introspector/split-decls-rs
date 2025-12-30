// Generated macro for configure_gdb (function)
macro_rules! Depcrate_debuggersconfigure_gdb {
() => {
// Module: crate::debuggers
// Provides: {"configure_gdb"}
// Dependencies: {}
pub (crate) fn configure_gdb (config : & Config) -> Option < Arc < Config > > { config . gdb_version ? ; if config . matches_env ("msvc") { return None ; } if config . remote_test_client . is_some () && ! config . target . contains ("android") { println ! ("WARNING: debuginfo tests are not available when \
             testing with remote") ; return None ; } if config . target . contains ("android") { println ! ("{} debug-info test uses tcp 5039 port.\
             please reserve it" , config . target) ; unsafe { env :: set_var ("RUST_TEST_THREADS" , "1") } ; } Some (Arc :: new (Config { debugger : Some (Debugger :: Gdb) , .. config . clone () })) }
};
}
