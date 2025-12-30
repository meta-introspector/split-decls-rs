// Generated macro for impl_910 (impl)
macro_rules! Depcrate_iter_noopimpl_910 {
() => {
// Module: crate::iter::noop
// Provides: {"impl_910"}
// Dependencies: {}
impl < T > Consumer < T > for NoopConsumer { type Folder = NoopConsumer ; type Reducer = NoopReducer ; type Result = () ; fn split_at (self , _index : usize) -> (Self , Self , NoopReducer) { (NoopConsumer , NoopConsumer , NoopReducer) } fn into_folder (self) -> Self { self } fn full (& self) -> bool { false } }
};
}
