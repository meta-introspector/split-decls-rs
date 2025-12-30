// Generated macro for guard (module)
macro_rules! Depcrate_rtguard {
() => {
// Module: crate::rt
// Provides: {"guard"}
// Dependencies: {}
pub mod guard { use crate :: is_generator ; use crate :: rt :: ContextStack ; use crate :: stack :: sys :: page_size ; use std :: ops :: Range ; pub type Guard = Range < usize > ; pub fn current () -> Guard { assert ! (is_generator ()) ; let guard = unsafe { (* (* ContextStack :: current () . root) . child) . stack_guard } ; guard . 0 - page_size () .. guard . 1 } }
};
}
