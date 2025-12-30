// Generated macro for impl_204 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_204 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_204"}
// Dependencies: {}
impl < 'a , S : ScalarValue > IntoIterator for LookAheadList < 'a , S > { type Item = BorrowedSpanning < 'a , LookAheadValue < 'a , S > > ; type IntoIter = look_ahead_list :: Iter < 'a , S > ; fn into_iter (self) -> Self :: IntoIter { (& self) . into_iter () } }
};
}
