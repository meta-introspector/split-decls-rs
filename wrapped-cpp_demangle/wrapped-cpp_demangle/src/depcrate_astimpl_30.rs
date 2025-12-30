// Generated macro for impl_30 (impl)
macro_rules! Depcrate_astimpl_30 {
() => {
// Module: crate::ast
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (feature = "logging")] impl Drop for AutoLogDemangle { fn drop (& mut self) { LOG_DEPTH . with (| depth | { * depth . borrow_mut () -= 1 ; let indent : String = (0 .. * depth . borrow () * 4) . map (| _ | ' ') . collect () ; log ! ("{})" , indent) ; }) ; } }
};
}
