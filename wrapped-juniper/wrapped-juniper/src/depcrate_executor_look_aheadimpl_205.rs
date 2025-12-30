// Generated macro for impl_205 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_205 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'a , S : ScalarValue > IntoIterator for & LookAheadList < 'a , S > { type Item = BorrowedSpanning < 'a , LookAheadValue < 'a , S > > ; type IntoIter = look_ahead_list :: Iter < 'a , S > ; fn into_iter (self) -> Self :: IntoIter { look_ahead_list :: Iter { slice_iter : self . input_list . iter () , vars : self . vars , } } }
};
}
