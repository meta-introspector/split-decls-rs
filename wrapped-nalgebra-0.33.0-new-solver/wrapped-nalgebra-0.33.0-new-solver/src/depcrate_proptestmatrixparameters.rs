// Generated macro for MatrixParameters (struct)
macro_rules! Depcrate_proptestMatrixParameters {
() => {
// Module: crate::proptest
// Provides: {"MatrixParameters"}
// Dependencies: {}
# [doc = " Parameters for arbitrary matrix generation."] # [derive (Debug , Clone)] # [non_exhaustive] pub struct MatrixParameters < NParameters , R , C > { # [doc = " The range of rows that may be generated."] pub rows : DimRange < R > , # [doc = " The range of columns that may be generated."] pub cols : DimRange < C > , # [doc = " Parameters for the `Arbitrary` implementation of the scalar values."] pub value_parameters : NParameters , }
};
}
