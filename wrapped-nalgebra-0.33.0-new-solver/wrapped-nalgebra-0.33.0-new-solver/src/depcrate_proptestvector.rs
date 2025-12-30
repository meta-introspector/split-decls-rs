// Generated macro for vector (function)
macro_rules! Depcrate_proptestvector {
() => {
// Module: crate::proptest
// Provides: {"vector"}
// Dependencies: {}
# [doc = " Create a strategy to generate column vectors containing values drawn from the given strategy,"] # [doc = " with length in the provided range."] # [doc = ""] # [doc = " This is a convenience function for calling"] # [doc = " [`matrix(value_strategy, length, U1)`](crate::matrix) and should"] # [doc = " be used when you only want to generate column vectors, as it's simpler and makes the intent"] # [doc = " clear."] pub fn vector < D , ScalarStrategy > (value_strategy : ScalarStrategy , length : impl Into < DimRange < D > > ,) -> MatrixStrategy < ScalarStrategy , D , U1 > where ScalarStrategy : Strategy + Clone + 'static , ScalarStrategy :: Value : Scalar , D : Dim , DefaultAllocator : Allocator < D > , { matrix_ (value_strategy , length . into () , Const :: < 1 > . into ()) }
};
}
