// Generated macro for impl_596 (impl)
macro_rules! Depcrate_future_race_ok_tuple_errorimpl_596 {
() => {
// Module: crate::future::race_ok::tuple::error
// Provides: {"impl_596"}
// Dependencies: {}
impl < E , const N : usize > Deref for AggregateError < E , N > { type Target = [E ; N] ; fn deref (& self) -> & Self :: Target { & self . inner } }
};
}
