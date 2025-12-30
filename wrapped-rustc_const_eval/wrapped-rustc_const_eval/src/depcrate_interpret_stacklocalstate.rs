// Generated macro for LocalState (struct)
macro_rules! Depcrate_interpret_stackLocalState {
() => {
// Module: crate::interpret::stack
// Provides: {"LocalState"}
// Dependencies: {}
# [doc = " State of a local variable including a memoized layout"] # [derive (Clone)] pub struct LocalState < 'tcx , Prov : Provenance = CtfeProvenance > { value : LocalValue < Prov > , # [doc = " Don't modify if `Some`, this is only used to prevent computing the layout twice."] # [doc = " Avoids computing the layout of locals that are never actually initialized."] layout : Cell < Option < TyAndLayout < 'tcx > > > , }
};
}
