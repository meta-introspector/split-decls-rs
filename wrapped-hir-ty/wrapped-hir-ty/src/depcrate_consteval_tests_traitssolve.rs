// Generated macro for solve (function)
macro_rules! Depcrate_consteval_tests_traitssolve {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"solve"}
// Dependencies: {}
fn solve (db : & dyn HirDatabase , krate : Crate , block : Option < BlockId > , goal : & chalk_ir :: UCanonical < chalk_ir :: InEnvironment < chalk_ir :: Goal < Interner > > > ,) -> Option < chalk_solve :: Solution < Interner > > { let _p = tracing :: info_span ! ("solve" , ? krate , ? block) . entered () ; let context = ChalkContext { db , krate , block } ; tracing :: debug ! ("solve goal: {:?}" , goal) ; let mut solver = create_chalk_solver () ; let fuel = std :: cell :: Cell :: new (CHALK_SOLVER_FUEL) ; let should_continue = | | { db . unwind_if_revision_cancelled () ; let remaining = fuel . get () ; fuel . set (remaining - 1) ; if remaining == 0 { tracing :: debug ! ("fuel exhausted") ; } remaining > 0 } ; let mut solve = | | { let _ctx = if is_chalk_debug () || is_chalk_print () { Some (panic_context :: enter (format ! ("solving {goal:?}"))) } else { None } ; let solution = if is_chalk_print () { let logging_db = LoggingRustIrDatabaseLoggingOnDrop (LoggingRustIrDatabase :: new (context)) ; solver . solve_limited (& logging_db . 0 , goal , & should_continue) } else { solver . solve_limited (& context , goal , & should_continue) } ; tracing :: debug ! ("solve({:?}) => {:?}" , goal , solution) ; solution } ; if is_chalk_debug () { crate :: tls :: set_current_program (db , solve) } else { solve () } }
};
}
