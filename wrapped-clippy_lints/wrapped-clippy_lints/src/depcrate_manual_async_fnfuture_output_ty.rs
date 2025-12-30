// Generated macro for future_output_ty (function)
macro_rules! Depcrate_manual_async_fnfuture_output_ty {
() => {
// Module: crate::manual_async_fn
// Provides: {"future_output_ty"}
// Dependencies: {}
fn future_output_ty < 'tcx > (trait_ref : & 'tcx TraitRef < 'tcx >) -> Option < & 'tcx Ty < 'tcx > > { if let Some (segment) = trait_ref . path . segments . last () && let Some (args) = segment . args && let [constraint] = args . constraints && constraint . ident . name == sym :: Output && let Some (output) = constraint . ty () { return Some (output) ; } None }
};
}
