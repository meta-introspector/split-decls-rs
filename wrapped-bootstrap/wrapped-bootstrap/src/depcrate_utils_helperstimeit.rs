// Generated macro for timeit (function)
macro_rules! Depcrate_utils_helperstimeit {
() => {
// Module: crate::utils::helpers
// Provides: {"timeit"}
// Dependencies: {}
# [doc = " Returns an RAII structure that prints out how long it took to drop."] pub fn timeit (builder : & Builder < '_ >) -> TimeIt { TimeIt (builder . config . dry_run () , Instant :: now ()) }
};
}
