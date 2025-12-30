// Generated macro for BatchInvert (trait)
macro_rules! Depcrate_batchBatchInvert {
() => {
// Module: crate::batch
// Provides: {"BatchInvert"}
// Dependencies: {}
# [doc = " Extension trait for iterators over mutable field elements which allows those field"] # [doc = " elements to be inverted in a batch."] # [doc = ""] # [doc = " `I: IntoIterator<Item = &'a mut F: Field + ConstantTimeEq>` implements this trait when"] # [doc = " the `alloc` feature flag is enabled."] # [doc = ""] # [doc = " For non-allocating contexts, see the [`BatchInverter`] struct."] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub trait BatchInvert < F : Field > { # [doc = " Consumes this iterator and inverts each field element (when nonzero). Zero-valued"] # [doc = " elements are left as zero."] # [doc = ""] # [doc = " Returns the inverse of the product of all nonzero field elements."] fn batch_invert (self) -> F ; }
};
}
