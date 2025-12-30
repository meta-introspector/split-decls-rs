// Generated macro for dimrange_rangefrom_usize (function)
macro_rules! Depcrate_base_indexingdimrange_rangefrom_usize {
() => {
// Module: crate::base::indexing
// Provides: {"dimrange_rangefrom_usize"}
// Dependencies: {}
# [test] fn dimrange_rangefrom_usize () { assert ! (! DimRange :: contained_by (& (0 ..) , Const ::< 0 >)) ; assert ! (! DimRange :: contained_by (& (0 ..) , Const ::< 0 >)) ; assert ! (DimRange :: contained_by (& (0 ..) , Const ::< 1 >)) ; assert ! (DimRange :: contained_by (& ((usize :: MAX - 1) ..) , Dyn (usize :: MAX))) ; assert_eq ! (DimRange :: length (& ((usize :: MAX - 1) ..) , Dyn (usize :: MAX)) , Dyn (1)) ; assert_eq ! (DimRange :: length (& (usize :: MAX ..) , Dyn (usize :: MAX)) , Dyn (0)) ; }
};
}
