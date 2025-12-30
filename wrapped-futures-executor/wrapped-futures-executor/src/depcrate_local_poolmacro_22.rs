// Generated macro for macro_22 (macro)
macro_rules! Depcrate_local_poolmacro_22 {
() => {
// Module: crate::local_pool
// Provides: {"macro_22"}
// Dependencies: {}
std :: thread_local ! { static CURRENT_THREAD_NOTIFY : Arc < ThreadNotify > = Arc :: new (ThreadNotify { thread : thread :: current () , unparked : AtomicBool :: new (false) , }) ; }
};
}
