// Generated macro for DebuggerContext (struct)
macro_rules! DepcrateDebuggerContext {
() => {
// Module: crate
// Provides: {"DebuggerContext"}
// Dependencies: {}
# [doc = " Debugger for pest grammars."] pub struct DebuggerContext { handle : Option < JoinHandle < () > > , is_done : Arc < AtomicBool > , grammar : Option < Vec < OptimizedRule > > , input : Option < String > , breakpoints : Arc < Mutex < HashSet < String > > > , }
};
}
