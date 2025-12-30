// Generated macro for impl_566 (impl)
macro_rules! Depcrate_future_race_ok_array_errorimpl_566 {
() => {
// Module: crate::future::race_ok::array::error
// Provides: {"impl_566"}
// Dependencies: {}
impl < E , const N : usize > Deref for AggregateError < E , N > { type Target = [E ; N] ; fn deref (& self) -> & Self :: Target { & self . inner } }
};
}
