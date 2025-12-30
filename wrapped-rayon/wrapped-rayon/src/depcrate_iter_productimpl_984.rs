// Generated macro for impl_984 (impl)
macro_rules! Depcrate_iter_productimpl_984 {
() => {
// Module: crate::iter::product
// Provides: {"impl_984"}
// Dependencies: {}
impl < P , T > UnindexedConsumer < T > for ProductConsumer < P > where P : Send + Product < T > + Product , { fn split_off_left (& self) -> Self { ProductConsumer :: new () } fn to_reducer (& self) -> Self :: Reducer { ProductConsumer :: new () } }
};
}
