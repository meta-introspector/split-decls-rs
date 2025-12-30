// Generated macro for impl_214 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_214 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_214"}
// Dependencies: {}
impl < 'a , S : ScalarValue > IntoIterator for & LookAheadObject < 'a , S > { type Item = (BorrowedSpanning < 'a , & 'a str > , BorrowedSpanning < 'a , LookAheadValue < 'a , S > > ,) ; type IntoIter = look_ahead_object :: Iter < 'a , S > ; fn into_iter (self) -> Self :: IntoIter { look_ahead_object :: Iter { slice_iter : self . input_object . iter () , vars : self . vars , } } }
};
}
