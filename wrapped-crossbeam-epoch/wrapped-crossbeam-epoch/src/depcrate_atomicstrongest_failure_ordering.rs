// Generated macro for strongest_failure_ordering (function)
macro_rules! Depcrate_atomicstrongest_failure_ordering {
() => {
// Module: crate::atomic
// Provides: {"strongest_failure_ordering"}
// Dependencies: {}
# [doc = " Given ordering for the success case in a compare-exchange operation, returns the strongest"] # [doc = " appropriate ordering for the failure case."] # [cfg (miri)] # [inline] fn strongest_failure_ordering (order : Ordering) -> Ordering { use Ordering :: * ; match order { Relaxed | Release => Relaxed , Acquire | AcqRel => Acquire , _ => SeqCst , } }
};
}
