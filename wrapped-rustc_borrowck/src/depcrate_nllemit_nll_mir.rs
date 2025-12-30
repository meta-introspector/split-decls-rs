// Generated macro for emit_nll_mir (function)
macro_rules! Depcrate_nllemit_nll_mir {
() => {
// Module: crate::nll
// Provides: {"emit_nll_mir"}
// Dependencies: {}
# [doc = " Produces the actual NLL MIR sections to emit during the dumping process."] pub (crate) fn emit_nll_mir < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > , borrow_set : & BorrowSet < 'tcx > , pass_where : PassWhere , out : & mut dyn io :: Write ,) -> io :: Result < () > { match pass_where { PassWhere :: BeforeCFG => { regioncx . dump_mir (tcx , out) ? ; writeln ! (out , "|") ? ; if let Some (closure_region_requirements) = closure_region_requirements { writeln ! (out , "| Free Region Constraints") ? ; for_each_region_constraint (tcx , closure_region_requirements , & mut | msg | { writeln ! (out , "| {msg}") }) ? ; writeln ! (out , "|") ? ; } if borrow_set . len () > 0 { writeln ! (out , "| Borrows") ? ; for (borrow_idx , borrow_data) in borrow_set . iter_enumerated () { writeln ! (out , "| {:?}: issued at {:?} in {:?}" , borrow_idx , borrow_data . reserve_location , borrow_data . region) ? ; } writeln ! (out , "|") ? ; } } PassWhere :: BeforeLocation (_) => { } PassWhere :: AfterTerminator (_) => { } PassWhere :: BeforeBlock (_) | PassWhere :: AfterLocation (_) | PassWhere :: AfterCFG => { } } Ok (()) }
};
}
