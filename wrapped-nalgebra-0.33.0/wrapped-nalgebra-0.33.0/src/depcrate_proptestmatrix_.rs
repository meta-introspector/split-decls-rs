// Generated macro for matrix_ (function)
macro_rules! Depcrate_proptestmatrix_ {
() => {
// Module: crate::proptest
// Provides: {"matrix_"}
// Dependencies: {}
# [doc = " Same as `matrix`, but without the additional anonymous generic types"] fn matrix_ < R , C , ScalarStrategy > (value_strategy : ScalarStrategy , rows : DimRange < R > , cols : DimRange < C > ,) -> MatrixStrategy < ScalarStrategy , R , C > where ScalarStrategy : Strategy + Clone + 'static , ScalarStrategy :: Value : Scalar , R : Dim , C : Dim , DefaultAllocator : Allocator < R , C > , { let nrows = rows . lower_bound () . value () ..= rows . upper_bound () . value () ; let ncols = cols . lower_bound () . value () ..= cols . upper_bound () . value () ; let strategy = nrows . prop_flat_map (move | nrows | (Just (nrows) , ncols . clone ())) . prop_flat_map (move | (nrows , ncols) | { (Just (nrows) , Just (ncols) , vec (value_strategy . clone () , nrows * ncols) ,) }) . prop_map (| (nrows , ncols , values) | { OMatrix :: from_iterator_generic (R :: from_usize (nrows) , C :: from_usize (ncols) , values) }) . boxed () ; MatrixStrategy { strategy } }
};
}
