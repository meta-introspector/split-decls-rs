// Generated macro for matrix (function)
macro_rules! Depcrate_proptestmatrix {
() => {
// Module: crate::proptest
// Provides: {"matrix"}
// Dependencies: {}
# [doc = " Create a strategy to generate matrices containing values drawn from the given strategy,"] # [doc = " with rows and columns in the provided ranges."] # [doc = ""] # [doc = " ## Examples"] # [doc = " ```"] # [doc = " use nalgebra::proptest::matrix;"] # [doc = " use nalgebra::{OMatrix, Const, Dyn};"] # [doc = " use proptest::prelude::*;"] # [doc = ""] # [doc = " proptest! {"] # [doc = "     # /*"] # [doc = "     #[test]"] # [doc = "     # */"] # [doc = "     fn my_test(a in matrix(0 .. 5i32, Const::<3>, 0 ..= 5)) {"] # [doc = "         // Let's make sure we've got the correct type first"] # [doc = "         let a: OMatrix<_, Const::<3>, Dyn> = a;"] # [doc = "         prop_assert!(a.nrows() == 3);"] # [doc = "         prop_assert!(a.ncols() <= 5);"] # [doc = "         prop_assert!(a.iter().all(|x_ij| *x_ij >= 0 && *x_ij < 5));"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() { my_test(); }"] # [doc = " ```"] # [doc = ""] # [doc = " ## Limitations"] # [doc = " The current implementation has some limitations that lead to suboptimal shrinking behavior."] # [doc = " See the [module-level documentation](index.html) for more."] pub fn matrix < R , C , ScalarStrategy > (value_strategy : ScalarStrategy , rows : impl Into < DimRange < R > > , cols : impl Into < DimRange < C > > ,) -> MatrixStrategy < ScalarStrategy , R , C > where ScalarStrategy : Strategy + Clone + 'static , ScalarStrategy :: Value : Scalar , R : Dim , C : Dim , DefaultAllocator : Allocator < R , C > , { matrix_ (value_strategy , rows . into () , cols . into ()) }
};
}
