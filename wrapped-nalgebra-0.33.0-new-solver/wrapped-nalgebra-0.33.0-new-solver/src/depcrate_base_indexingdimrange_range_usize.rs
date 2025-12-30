// Generated macro for dimrange_range_usize (function)
macro_rules! Depcrate_base_indexingdimrange_range_usize {
() => {
// Module: crate::base::indexing
// Provides: {"dimrange_range_usize"}
// Dependencies: {}
# [test] fn dimrange_range_usize () { assert ! (! DimRange :: contained_by (& (0 .. 0) , Const ::< 0 >)) ; assert ! (! DimRange :: contained_by (& (0 .. 1) , Const ::< 0 >)) ; assert ! (DimRange :: contained_by (& (0 .. 1) , Const ::< 1 >)) ; assert ! (DimRange :: contained_by (& ((usize :: MAX - 1) .. usize :: MAX) , Dyn (usize :: MAX))) ; assert_eq ! (DimRange :: length (& ((usize :: MAX - 1) .. usize :: MAX) , Dyn (usize :: MAX)) , Dyn (1)) ; assert_eq ! (DimRange :: length (& (usize :: MAX .. (usize :: MAX - 1)) , Dyn (usize :: MAX)) , Dyn (0)) ; assert_eq ! (DimRange :: length (& (usize :: MAX .. usize :: MAX) , Dyn (usize :: MAX)) , Dyn (0)) ; }
};
}
