// Generated macro for impl_430 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_430 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_430"}
// Dependencies: {}
impl < 'c , T : Send + 'c > Folder < T > for CollectResult < 'c , T > { type Result = Self ; fn consume (mut self , item : T) -> Self { assert ! (self . initialized_len < self . total_len , "too many values pushed to consumer") ; unsafe { self . start . 0 . add (self . initialized_len) . write (item) ; self . initialized_len += 1 ; } self } fn complete (self) -> Self :: Result { self } fn full (& self) -> bool { false } }
};
}
