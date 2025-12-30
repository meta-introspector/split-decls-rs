// Generated macro for impl_428 (impl)
macro_rules! Depcrate_concurrency_data_raceimpl_428 {
() => {
// Module: crate::concurrency::data_race
// Provides: {"impl_428"}
// Dependencies: {}
impl ThreadClockSet { # [doc = " Apply the effects of a release fence to this"] # [doc = " set of thread vector clocks."] # [inline] fn apply_release_fence (& mut self) { self . fence_release . clone_from (& self . clock) ; } # [doc = " Apply the effects of an acquire fence to this"] # [doc = " set of thread vector clocks."] # [inline] fn apply_acquire_fence (& mut self) { self . clock . join (& self . fence_acquire) ; } # [doc = " Increment the happens-before clock at a"] # [doc = " known index."] # [inline] fn increment_clock (& mut self , index : VectorIdx , current_span : Span) { self . clock . increment_index (index , current_span) ; } # [doc = " Join the happens-before clock with that of"] # [doc = " another thread, used to model thread join"] # [doc = " operations."] fn join_with (& mut self , other : & ThreadClockSet) { self . clock . join (& other . clock) ; } }
};
}
