// Generated macro for impl_1636 (impl)
macro_rules! Depcrate_stream_select_with_strategyimpl_1636 {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"impl_1636"}
// Dependencies: {}
impl PollNext { # [doc = " Toggle the value and return the old one."] # [must_use] pub fn toggle (& mut self) -> Self { let old = * self ; * self = self . other () ; old } fn other (& self) -> Self { match self { Self :: Left => Self :: Right , Self :: Right => Self :: Left , } } }
};
}
