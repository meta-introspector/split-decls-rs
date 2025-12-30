// Generated macro for impl_3039 (impl)
macro_rules! Depcrate_proptestimpl_3039 {
() => {
// Module: crate::proptest
// Provides: {"impl_3039"}
// Dependencies: {}
impl < NStrategy , R , C > Strategy for MatrixStrategy < NStrategy , R , C > where NStrategy : Strategy , NStrategy :: Value : Scalar , R : Dim , C : Dim , DefaultAllocator : Allocator < R , C > , { type Tree = MatrixValueTree < NStrategy :: Value , R , C > ; type Value = OMatrix < NStrategy :: Value , R , C > ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let underlying_tree = self . strategy . new_tree (runner) ? ; Ok (MatrixValueTree { value_tree : underlying_tree , }) } }
};
}
