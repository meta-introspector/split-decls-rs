// Generated macro for LateLintPassFactory (type)
macro_rules! Depcrate_contextLateLintPassFactory {
() => {
// Module: crate::context
// Provides: {"LateLintPassFactory"}
// Dependencies: {}
type LateLintPassFactory = dyn for < 'tcx > Fn (TyCtxt < 'tcx >) -> LateLintPassObject < 'tcx > + sync :: DynSend + sync :: DynSync ;
};
}
