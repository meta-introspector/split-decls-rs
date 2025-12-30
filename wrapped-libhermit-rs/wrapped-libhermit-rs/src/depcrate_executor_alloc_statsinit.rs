// Generated macro for init (function)
macro_rules! Depcrate_executor_alloc_statsinit {
() => {
// Module: crate::executor::alloc_stats
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { info ! ("Spawning allocation stats printing task") ; spawn (print_alloc_stats ()) ; }
};
}
