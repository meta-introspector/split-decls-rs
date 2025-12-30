// Generated macro for impl_228 (impl)
macro_rules! Depcrate_const_eval_eval_queriesimpl_228 {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'tcx > InterpretationResult < 'tcx > for ConstAlloc < 'tcx > { fn make_result (mplace : MPlaceTy < 'tcx > , _ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self { ConstAlloc { alloc_id : mplace . ptr () . provenance . unwrap () . alloc_id () , ty : mplace . layout . ty } } }
};
}
