// Generated macro for impl_721 (impl)
macro_rules! Depcrate_concurrency_genmc_shimsimpl_721 {
() => {
// Module: crate::concurrency::genmc::shims
// Provides: {"impl_721"}
// Dependencies: {}
impl GenmcCtx { # [doc = " Handle a user thread getting blocked."] # [doc = " This may happen due to an manual `assume` statement added by a user"] # [doc = " or added by some automated program transformation, e.g., for spinloops."] fn handle_assume_block < 'tcx > (& self , machine : & MiriMachine < 'tcx > , assume_type : AssumeType ,) -> InterpResult < 'tcx > { debug ! ("GenMC: assume statement, blocking active thread.") ; self . handle . borrow_mut () . pin_mut () . handle_assume_block (self . active_thread_genmc_tid (machine) , assume_type) ; interp_ok (()) } }
};
}
