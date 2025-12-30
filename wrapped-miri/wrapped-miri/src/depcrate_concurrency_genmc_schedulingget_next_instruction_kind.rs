// Generated macro for get_next_instruction_kind (function)
macro_rules! Depcrate_concurrency_genmc_schedulingget_next_instruction_kind {
() => {
// Module: crate::concurrency::genmc::scheduling
// Provides: {"get_next_instruction_kind"}
// Dependencies: {}
# [doc = " Check if a call or tail-call could have atomic load semantics."] fn get_next_instruction_kind < 'tcx > (ecx : & InterpCx < 'tcx , MiriMachine < 'tcx > > ,) -> InterpResult < 'tcx , NextInstrKind > { use NextInstrKind :: * ; let thread_manager = & ecx . machine . threads ; if ! thread_manager . active_thread_ref () . is_enabled () { return interp_ok (MaybeAtomic (ActionKind :: Load)) ; } let Some (frame) = thread_manager . active_thread_stack () . last () else { return interp_ok (NonAtomic) ; } ; let Either :: Left (loc) = frame . current_loc () else { return interp_ok (NonAtomic) ; } ; let basic_block = & frame . body () . basic_blocks [loc . block] ; if let Some (_statement) = basic_block . statements . get (loc . statement_index) { return interp_ok (NonAtomic) ; } match & basic_block . terminator () . kind { TerminatorKind :: Call { func , .. } | TerminatorKind :: TailCall { func , .. } => get_function_kind (ecx , func . ty (& frame . body () . local_decls , * ecx . tcx)) , _ => interp_ok (NonAtomic) , } }
};
}
