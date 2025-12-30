// Generated macro for configure_lldb (function)
macro_rules! Depcrate_debuggersconfigure_lldb {
() => {
// Module: crate::debuggers
// Provides: {"configure_lldb"}
// Dependencies: {}
pub (crate) fn configure_lldb (config : & Config) -> Option < Arc < Config > > { config . lldb_python_dir . as_ref () ? ; Some (Arc :: new (Config { debugger : Some (Debugger :: Lldb) , .. config . clone () })) }
};
}
