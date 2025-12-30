// Generated macro for PlacePrinter (struct)
macro_rules! Depcrate_interpret_eval_contextPlacePrinter {
() => {
// Module: crate::interpret::eval_context
// Provides: {"PlacePrinter"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Helper struct for the `dump_place` function."] pub struct PlacePrinter < 'a , 'tcx , M : Machine < 'tcx > > { ecx : & 'a InterpCx < 'tcx , M > , place : Place < M :: Provenance > , }
};
}
