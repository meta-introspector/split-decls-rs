// Generated macro for product (function)
macro_rules! Depcrate_iter_productproduct {
() => {
// Module: crate::iter::product
// Provides: {"product"}
// Dependencies: {}
pub (super) fn product < PI , P > (pi : PI) -> P where PI : ParallelIterator , P : Send + Product < PI :: Item > + Product , { pi . drive_unindexed (ProductConsumer :: new ()) }
};
}
