// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Default for DebuggerContext { fn default () -> Self { Self { handle : None , is_done : Arc :: new (AtomicBool :: new (false)) , grammar : None , input : None , breakpoints : Arc :: new (Mutex :: new (HashSet :: new ())) , } } }
};
}
