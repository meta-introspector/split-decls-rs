// Generated macro for impl_1213 (impl)
macro_rules! Depcrate_iter_unzipimpl_1213 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1213"}
// Dependencies: {}
impl < 'a , T , OP , CA , CB > UnindexedConsumer < T > for UnzipConsumer < 'a , OP , CA , CB > where OP : UnzipOp < T > , CA : UnindexedConsumer < OP :: Left > , CB : UnindexedConsumer < OP :: Right > , { fn split_off_left (& self) -> Self { UnzipConsumer { op : self . op , left : self . left . split_off_left () , right : self . right . split_off_left () , } } fn to_reducer (& self) -> Self :: Reducer { UnzipReducer { left : self . left . to_reducer () , right : self . right . to_reducer () , } } }
};
}
