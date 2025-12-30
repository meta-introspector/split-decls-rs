// Generated macro for impl_213 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_213 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'a , S : ScalarValue > IntoIterator for LookAheadObject < 'a , S > { type Item = (BorrowedSpanning < 'a , & 'a str > , BorrowedSpanning < 'a , LookAheadValue < 'a , S > > ,) ; type IntoIter = look_ahead_object :: Iter < 'a , S > ; fn into_iter (self) -> Self :: IntoIter { (& self) . into_iter () } }
};
}
