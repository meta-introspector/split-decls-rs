// Generated macro for impl_1643 (impl)
macro_rules! Depcrate_stream_select_with_strategyimpl_1643 {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"impl_1643"}
// Dependencies: {}
impl < St1 , St2 , Clos , State > FusedStream for SelectWithStrategy < St1 , St2 , Clos , State > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , Clos : FnMut (& mut State) -> PollNext , { fn is_terminated (& self) -> bool { matches ! (self . internal_state , InternalState :: BothFinished) } }
};
}
