// Generated macro for isolation_abort_error (function)
macro_rules! Depcrate_helpersisolation_abort_error {
() => {
// Module: crate::helpers
// Provides: {"isolation_abort_error"}
// Dependencies: {}
pub fn isolation_abort_error < 'tcx > (name : & str) -> InterpResult < 'tcx > { throw_machine_stop ! (TerminationInfo :: UnsupportedInIsolation (format ! ("{name} not available when isolation is enabled" ,))) }
};
}
