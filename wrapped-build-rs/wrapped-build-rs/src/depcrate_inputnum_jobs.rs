// Generated macro for num_jobs (function)
macro_rules! Depcrate_inputnum_jobs {
() => {
// Module: crate::input
// Provides: {"num_jobs"}
// Dependencies: {}
# [doc = " The parallelism specified as the top-level parallelism."] # [doc = ""] # [doc = " This can be useful to"] # [doc = " pass a `-j` parameter to a system like `make`. Note that care should be taken"] # [doc = " when interpreting this value. For historical purposes this is still provided"] # [doc = " but Cargo, for example, does not need to run `make -j`, and instead can set the"] # [doc = " `MAKEFLAGS` env var to the content of `CARGO_MAKEFLAGS` to activate the use of"] # [doc = " Cargo’s GNU Make compatible [jobserver] for sub-make invocations."] # [doc = ""] # [doc = " [jobserver]: https://www.gnu.org/software/make/manual/html_node/Job-Slots.html"] # [track_caller] pub fn num_jobs () -> u32 { to_parsed (var_or_panic ("NUM_JOBS")) }
};
}
