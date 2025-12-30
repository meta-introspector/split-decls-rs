// Generated macro for macro_643 (macro)
macro_rules! Depcrate_stats_rand_utilmacro_643 {
() => {
// Module: crate::stats::rand_util
// Provides: {"macro_643"}
// Dependencies: {}
thread_local ! { static SEED_RAND : RefCell < Rand64 > = RefCell :: new (Rand64 :: new (SystemTime :: now () . duration_since (UNIX_EPOCH) . expect ("Time went backwards") . as_millis ())) ; }
};
}
