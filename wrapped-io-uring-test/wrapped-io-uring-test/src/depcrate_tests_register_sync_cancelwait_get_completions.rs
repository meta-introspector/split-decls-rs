// Generated macro for wait_get_completions (function)
macro_rules! Depcrate_tests_register_sync_cancelwait_get_completions {
() => {
// Module: crate::tests::register_sync_cancel
// Provides: {"wait_get_completions"}
// Dependencies: {}
# [doc = " Blocks for a short amount of time, waiting for completions to arrive."] # [doc = ""] # [doc = " Returns all completions that have arrived."] fn wait_get_completions < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , want : usize ,) -> io :: Result < Vec < cqueue :: Entry > > { let ts = types :: Timespec :: new () . nsec (1_000_000) . sec (0) ; let args = types :: SubmitArgs :: new () . timespec (& ts) ; ring . submitter () . submit_with_args (want , & args) ? ; Ok (ring . completion () . map (Into :: into) . collect :: < Vec < cqueue :: Entry > > ()) }
};
}
