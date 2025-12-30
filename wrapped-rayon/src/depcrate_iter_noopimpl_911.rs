// Generated macro for impl_911 (impl)
macro_rules! Depcrate_iter_noopimpl_911 {
() => {
// Module: crate::iter::noop
// Provides: {"impl_911"}
// Dependencies: {}
impl < T > Folder < T > for NoopConsumer { type Result = () ; fn consume (self , _item : T) -> Self { self } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { iter . into_iter () . for_each (drop) ; self } fn complete (self) { } fn full (& self) -> bool { false } }
};
}
