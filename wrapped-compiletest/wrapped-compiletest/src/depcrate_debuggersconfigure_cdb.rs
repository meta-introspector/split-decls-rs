// Generated macro for configure_cdb (function)
macro_rules! Depcrate_debuggersconfigure_cdb {
() => {
// Module: crate::debuggers
// Provides: {"configure_cdb"}
// Dependencies: {}
pub (crate) fn configure_cdb (config : & Config) -> Option < Arc < Config > > { config . cdb . as_ref () ? ; Some (Arc :: new (Config { debugger : Some (Debugger :: Cdb) , .. config . clone () })) }
};
}
