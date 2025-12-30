// Generated macro for proc_macros_for_crate (function)
macro_rules! Depcrate_proc_macroproc_macros_for_crate {
() => {
// Module: crate::proc_macro
// Provides: {"proc_macros_for_crate"}
// Dependencies: {}
pub (crate) fn proc_macros_for_crate (db : & dyn ExpandDatabase , krate : Crate ,) -> Option < Arc < CrateProcMacros > > { db . proc_macros () . get (krate) }
};
}
