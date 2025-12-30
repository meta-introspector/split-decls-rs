// Generated macro for RUN_ID (static)
macro_rules! Depcrate_snapshotRUN_ID {
() => {
// Module: crate::snapshot
// Provides: {"RUN_ID"}
// Dependencies: {}
static RUN_ID : Lazy < String > = Lazy :: new (| | { if let Ok (run_id) = env :: var ("NEXTEST_RUN_ID") { run_id } else { let d = SystemTime :: now () . duration_since (UNIX_EPOCH) . unwrap () ; format ! ("{}-{}" , d . as_secs () , d . subsec_nanos ()) } }) ;
};
}
