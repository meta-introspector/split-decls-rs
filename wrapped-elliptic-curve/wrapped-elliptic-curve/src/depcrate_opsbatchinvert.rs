// Generated macro for BatchInvert (trait)
macro_rules! Depcrate_opsBatchInvert {
() => {
// Module: crate::ops
// Provides: {"BatchInvert"}
// Dependencies: {}
# [doc = " Perform a batched inversion on a sequence of field elements (i.e. base field elements or scalars)"] # [doc = " at an amortized cost that should be practically as efficient as a single inversion."] pub trait BatchInvert < FieldElements : ? Sized > { # [doc = " The output of batch inversion. A container of field elements."] type Output ; # [doc = " Invert a batch of field elements."] fn batch_invert (field_elements : FieldElements) -> < Self as BatchInvert < FieldElements > > :: Output ; }
};
}
