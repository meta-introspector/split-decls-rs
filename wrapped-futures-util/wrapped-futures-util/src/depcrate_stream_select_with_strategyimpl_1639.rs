// Generated macro for impl_1639 (impl)
macro_rules! Depcrate_stream_select_with_strategyimpl_1639 {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"impl_1639"}
// Dependencies: {}
impl InternalState { fn finish (& mut self , ps : PollNext) { match (& self , ps) { (Self :: Start , PollNext :: Left) => { * self = Self :: LeftFinished ; } (Self :: Start , PollNext :: Right) => { * self = Self :: RightFinished ; } (Self :: LeftFinished , PollNext :: Right) | (Self :: RightFinished , PollNext :: Left) => { * self = Self :: BothFinished ; } _ => { } } } }
};
}
