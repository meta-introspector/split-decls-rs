// Generated macro for impl_733 (impl)
macro_rules! Depcrate_iter_for_eachimpl_733 {
() => {
// Module: crate::iter::for_each
// Provides: {"impl_733"}
// Dependencies: {}
impl < 'f , F , T > Consumer < T > for ForEachConsumer < 'f , F > where F : Fn (T) + Sync , { type Folder = ForEachConsumer < 'f , F > ; type Reducer = NoopReducer ; type Result = () ; fn split_at (self , _index : usize) -> (Self , Self , NoopReducer) { (self . split_off_left () , self , NoopReducer) } fn into_folder (self) -> Self { self } fn full (& self) -> bool { false } }
};
}
