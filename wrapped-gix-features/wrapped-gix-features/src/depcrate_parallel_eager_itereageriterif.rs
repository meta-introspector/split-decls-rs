// Generated macro for EagerIterIf (enum)
macro_rules! Depcrate_parallel_eager_iterEagerIterIf {
() => {
// Module: crate::parallel::eager_iter
// Provides: {"EagerIterIf"}
// Dependencies: {}
# [doc = " An conditional `EagerIter`, which may become a just-in-time iterator running in the main thread depending on a condition."] pub enum EagerIterIf < I : Iterator > { # [doc = " A separate thread will eagerly evaluate iterator `I`."] Eager (EagerIter < I >) , # [doc = " The current thread evaluates `I`."] OnDemand (I) , }
};
}
