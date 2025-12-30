// Generated macro for run_genmc_mode (function)
macro_rules! Depcrate_concurrency_genmc_runrun_genmc_mode {
() => {
// Module: crate::concurrency::genmc::run
// Provides: {"run_genmc_mode"}
// Dependencies: {}
# [doc = " Do a complete run of the program in GenMC mode."] # [doc = " This will call `eval_entry` multiple times, until either:"] # [doc = " - An error is detected (indicated by a `None` return value)"] # [doc = " - All possible executions are explored."] # [doc = ""] # [doc = " Returns `None` is an error is detected, or `Some(return_value)` with the return value of the last run of the program."] pub fn run_genmc_mode < 'tcx > (tcx : TyCtxt < 'tcx > , config : & MiriConfig , eval_entry : impl Fn (Rc < GenmcCtx >) -> Result < () , NonZeroI32 > ,) -> Result < () , NonZeroI32 > { if tcx . data_layout . endian != Endian :: Little || tcx . data_layout . pointer_size () . bits () != 64 { tcx . dcx () . fatal ("GenMC only supports 64bit little-endian targets") ; } let genmc_config = config . genmc_config . as_ref () . unwrap () ; if genmc_config . do_estimation { eprintln ! ("Estimating GenMC verification time...") ; run_genmc_mode_impl (config , & eval_entry , tcx , GenmcMode :: Estimation) ? ; } eprintln ! ("Running GenMC Verification...") ; run_genmc_mode_impl (config , & eval_entry , tcx , GenmcMode :: Verification) }
};
}
