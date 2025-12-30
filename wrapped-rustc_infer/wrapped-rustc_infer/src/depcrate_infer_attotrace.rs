// Generated macro for ToTrace (trait)
macro_rules! Depcrate_infer_atToTrace {
() => {
// Module: crate::infer::at
// Provides: {"ToTrace"}
// Dependencies: {}
pub trait ToTrace < 'tcx > : Relate < TyCtxt < 'tcx > > + Copy { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > ; }
};
}
