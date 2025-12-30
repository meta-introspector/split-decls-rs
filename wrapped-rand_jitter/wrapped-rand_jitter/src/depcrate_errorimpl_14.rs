// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl TimerError { fn description (& self) -> & 'static str { match * self { TimerError :: NoTimer => "no timer available" , TimerError :: CoarseTimer => "coarse timer" , TimerError :: NotMonotonic => "timer not monotonic" , TimerError :: TinyVariations => "time delta variations too small" , TimerError :: TooManyStuck => "too many stuck results" , TimerError :: __Nonexhaustive => unreachable ! () , } } }
};
}
