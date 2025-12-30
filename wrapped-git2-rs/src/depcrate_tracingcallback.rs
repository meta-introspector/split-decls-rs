// Generated macro for CALLBACK (static)
macro_rules! Depcrate_tracingCALLBACK {
() => {
// Module: crate::tracing
// Provides: {"CALLBACK"}
// Dependencies: {}
# [doc = " Use an atomic pointer to store the global tracing subscriber function."] static CALLBACK : AtomicPtr < () > = AtomicPtr :: new (std :: ptr :: null_mut ()) ;
};
}
