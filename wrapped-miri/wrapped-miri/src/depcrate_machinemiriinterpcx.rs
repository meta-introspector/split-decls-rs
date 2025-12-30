// Generated macro for MiriInterpCx (type)
macro_rules! Depcrate_machineMiriInterpCx {
() => {
// Module: crate::machine
// Provides: {"MiriInterpCx"}
// Dependencies: {}
# [doc = " A rustc InterpCx for Miri."] pub type MiriInterpCx < 'tcx > = InterpCx < 'tcx , MiriMachine < 'tcx > > ;
};
}
