// Generated macro for run_genmc_mode_impl (function)
macro_rules! Depcrate_concurrency_genmc_runrun_genmc_mode_impl {
() => {
// Module: crate::concurrency::genmc::run
// Provides: {"run_genmc_mode_impl"}
// Dependencies: {}
fn run_genmc_mode_impl < 'tcx > (config : & MiriConfig , eval_entry : & impl Fn (Rc < GenmcCtx >) -> Result < () , NonZeroI32 > , tcx : TyCtxt < 'tcx > , mode : GenmcMode ,) -> Result < () , NonZeroI32 > { let time_start = Instant :: now () ; let genmc_config = config . genmc_config . as_ref () . unwrap () ; let global_state = Arc :: new (GlobalState :: new (tcx . target_usize_max ())) ; let genmc_ctx = Rc :: new (GenmcCtx :: new (config , global_state , mode)) ; for rep in 0u64 .. { tracing :: info ! ("Miri-GenMC loop {}" , rep + 1) ; genmc_ctx . prepare_next_execution () ; if let Err (err) = eval_entry (genmc_ctx . clone ()) { genmc_ctx . print_genmc_output (genmc_config , tcx) ; return Err (err) ; } ; match genmc_ctx . handle_execution_end () { ExecutionEndResult :: Continue => continue , ExecutionEndResult :: Stop => { let elapsed_time_sec = Instant :: now () . duration_since (time_start) . as_secs_f64 () ; if mode == GenmcMode :: Estimation { genmc_ctx . print_estimation_output (genmc_config , elapsed_time_sec) ; } else { genmc_ctx . print_verification_output (genmc_config , elapsed_time_sec) ; } return Ok (()) ; } ExecutionEndResult :: Error (error) => { eprintln ! ("(GenMC) Error detected: {error}") ; genmc_ctx . print_genmc_output (genmc_config , tcx) ; return Err (NonZeroI32 :: new (rustc_driver :: EXIT_FAILURE) . unwrap ()) ; } } } unreachable ! () }
};
}
