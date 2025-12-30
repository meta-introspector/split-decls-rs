// Generated macro for macro_1640 (macro)
macro_rules! Depcrate_stream_select_with_strategymacro_1640 {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"macro_1640"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`select_with_strategy()`] function. See function docs for details."] # [must_use = "streams do nothing unless polled"] # [project = SelectWithStrategyProj] pub struct SelectWithStrategy < St1 , St2 , Clos , State > { # [pin] stream1 : St1 , # [pin] stream2 : St2 , internal_state : InternalState , state : State , clos : Clos , } }
};
}
