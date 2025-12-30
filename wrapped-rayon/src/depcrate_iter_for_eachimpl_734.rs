// Generated macro for impl_734 (impl)
macro_rules! Depcrate_iter_for_eachimpl_734 {
() => {
// Module: crate::iter::for_each
// Provides: {"impl_734"}
// Dependencies: {}
impl < 'f , F , T > Folder < T > for ForEachConsumer < 'f , F > where F : Fn (T) + Sync , { type Result = () ; fn consume (self , item : T) -> Self { (self . op) (item) ; self } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { iter . into_iter () . for_each (self . op) ; self } fn complete (self) { } fn full (& self) -> bool { false } }
};
}
