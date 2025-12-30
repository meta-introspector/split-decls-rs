// Generated macro for impl_200 (impl)
macro_rules! Depcrate_bridgeimpl_200 {
() => {
// Module: crate::bridge
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'a , T , M > Unmark for & 'a mut Marked < T , M > { type Unmarked = & 'a mut T ; fn unmark (self) -> Self :: Unmarked { & mut self . value } }
};
}
