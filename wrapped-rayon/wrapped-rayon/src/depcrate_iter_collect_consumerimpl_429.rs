// Generated macro for impl_429 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_429 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_429"}
// Dependencies: {}
impl < 'c , T : Send + 'c > Consumer < T > for CollectConsumer < 'c , T > { type Folder = CollectResult < 'c , T > ; type Reducer = CollectReducer ; type Result = CollectResult < 'c , T > ; fn split_at (self , index : usize) -> (Self , Self , CollectReducer) { let CollectConsumer { start , len , .. } = self ; unsafe { assert ! (index <= len) ; (CollectConsumer :: new (start . 0 , index) , CollectConsumer :: new (start . 0 . add (index) , len - index) , CollectReducer ,) } } fn into_folder (self) -> Self :: Folder { CollectResult { start : self . start , total_len : self . len , initialized_len : 0 , invariant_lifetime : PhantomData , } } fn full (& self) -> bool { false } }
};
}
