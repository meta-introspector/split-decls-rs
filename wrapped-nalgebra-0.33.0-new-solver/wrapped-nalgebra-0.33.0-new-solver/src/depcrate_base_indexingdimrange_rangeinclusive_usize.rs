// Generated macro for dimrange_rangeinclusive_usize (function)
macro_rules! Depcrate_base_indexingdimrange_rangeinclusive_usize {
() => {
// Module: crate::base::indexing
// Provides: {"dimrange_rangeinclusive_usize"}
// Dependencies: {}
# [test] fn dimrange_rangeinclusive_usize () { assert ! (! DimRange :: contained_by (& (0 ..= 0) , Const ::< 0 >)) ; assert ! (DimRange :: contained_by (& (0 ..= 0) , Const ::< 1 >)) ; assert ! (! DimRange :: contained_by (& (usize :: MAX ..= usize :: MAX) , Dyn (usize :: MAX))) ; assert ! (! DimRange :: contained_by (& ((usize :: MAX - 1) ..= usize :: MAX) , Dyn (usize :: MAX))) ; assert ! (DimRange :: contained_by (& ((usize :: MAX - 1) ..= (usize :: MAX - 1)) , Dyn (usize :: MAX))) ; assert_eq ! (DimRange :: length (& (0 ..= 0) , Const ::< 1 >) , Dyn (1)) ; assert_eq ! (DimRange :: length (& ((usize :: MAX - 1) ..= usize :: MAX) , Dyn (usize :: MAX)) , Dyn (2)) ; assert_eq ! (DimRange :: length (& (usize :: MAX ..= (usize :: MAX - 1)) , Dyn (usize :: MAX)) , Dyn (0)) ; assert_eq ! (DimRange :: length (& (usize :: MAX ..= usize :: MAX) , Dyn (usize :: MAX)) , Dyn (1)) ; }
};
}
