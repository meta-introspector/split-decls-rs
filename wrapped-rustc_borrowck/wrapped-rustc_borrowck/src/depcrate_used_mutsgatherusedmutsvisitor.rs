// Generated macro for GatherUsedMutsVisitor (struct)
macro_rules! Depcrate_used_mutsGatherUsedMutsVisitor {
() => {
// Module: crate::used_muts
// Provides: {"GatherUsedMutsVisitor"}
// Dependencies: {}
# [doc = " MIR visitor for collecting used mutable variables."] # [doc = " The 'visit lifetime represents the duration of the MIR walk."] struct GatherUsedMutsVisitor < 'a , 'b , 'infcx , 'tcx > { temporary_used_locals : FxIndexSet < Local > , never_initialized_mut_locals : & 'a mut FxIndexSet < Local > , mbcx : & 'a mut MirBorrowckCtxt < 'b , 'infcx , 'tcx > , }
};
}
