// Generated macro for report_leaks (function)
macro_rules! Depcrate_diagnosticsreport_leaks {
() => {
// Module: crate::diagnostics
// Provides: {"report_leaks"}
// Dependencies: {}
pub fn report_leaks < 'tcx > (ecx : & InterpCx < 'tcx , MiriMachine < 'tcx > > , leaks : Vec < (AllocId , MemoryKind , Allocation < Provenance , AllocExtra < 'tcx > , MiriAllocBytes >) > ,) { let mut any_pruned = false ; for (id , kind , alloc) in leaks { let mut title = format ! ("memory leaked: {id:?} ({}, size: {:?}, align: {:?})" , kind , alloc . size () . bytes () , alloc . align . bytes ()) ; let Some (backtrace) = alloc . extra . backtrace else { ecx . tcx . dcx () . err (title) ; continue ; } ; title . push_str (", allocated here:") ; let (backtrace , pruned) = prune_stacktrace (backtrace , & ecx . machine) ; any_pruned |= pruned ; report_msg (DiagLevel :: Error , title , vec ! [] , vec ! [] , vec ! [] , & backtrace , None , & ecx . machine ,) ; } if any_pruned { ecx . tcx . dcx () . note ("some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace" ,) ; } }
};
}
