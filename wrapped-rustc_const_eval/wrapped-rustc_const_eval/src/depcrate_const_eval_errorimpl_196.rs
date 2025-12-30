// Generated macro for impl_196 (impl)
macro_rules! Depcrate_const_eval_errorimpl_196 {
() => {
// Module: crate::const_eval::error
// Provides: {"impl_196"}
// Dependencies: {}
# [doc = " The errors become [`InterpErrorKind::MachineStop`] when being raised."] impl < 'tcx > Into < InterpErrorInfo < 'tcx > > for ConstEvalErrKind { fn into (self) -> InterpErrorInfo < 'tcx > { err_machine_stop ! (self) . into () } }
};
}
