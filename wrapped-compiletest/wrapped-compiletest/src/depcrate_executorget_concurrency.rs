// Generated macro for get_concurrency (function)
macro_rules! Depcrate_executorget_concurrency {
() => {
// Module: crate::executor
// Provides: {"get_concurrency"}
// Dependencies: {}
# [doc = " Determines the number of tests to run concurrently."] # [doc = ""] # [doc = " Copied from `get_concurrency` in libtest."] # [doc = ""] # [doc = " FIXME(#139660): After the libtest dependency is removed, consider making bootstrap specify the"] # [doc = " number of threads on the command-line, instead of propagating the `RUST_TEST_THREADS`"] # [doc = " environment variable."] fn get_concurrency () -> usize { if let Ok (value) = env :: var ("RUST_TEST_THREADS") { match value . parse :: < NonZero < usize > > () . ok () { Some (n) => n . get () , _ => panic ! ("RUST_TEST_THREADS is `{value}`, should be a positive integer.") , } } else { thread :: available_parallelism () . map (| n | n . get ()) . unwrap_or (1) } }
};
}
