// Generated macro for impl_985 (impl)
macro_rules! Depcrate_iter_productimpl_985 {
() => {
// Module: crate::iter::product
// Provides: {"impl_985"}
// Dependencies: {}
impl < P > Reducer < P > for ProductConsumer < P > where P : Send + Product , { fn reduce (self , left : P , right : P) -> P { mul (left , right) } }
};
}
