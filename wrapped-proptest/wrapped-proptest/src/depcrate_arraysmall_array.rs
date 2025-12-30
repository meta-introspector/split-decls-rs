// Generated macro for small_array (macro)
macro_rules! Depcrate_arraysmall_array {
() => {
// Module: crate::array
// Provides: {"small_array"}
// Dependencies: {}
macro_rules ! small_array { ($ n : tt $ uni : ident) => { # [doc = " Create a strategy to generate fixed-length arrays."] # [doc = ""] # [doc = " All values within the new strategy are generated using the given"] # [doc = " strategy. The length of the array corresponds to the suffix of the"] # [doc = " name of this function."] # [doc = ""] # [doc = " See [`UniformArrayStrategy`](struct.UniformArrayStrategy.html) for"] # [doc = " example usage."] pub fn $ uni < S : Strategy > (strategy : S ,) -> UniformArrayStrategy < S , [S :: Value ; $ n] > { UniformArrayStrategy { strategy , _marker : PhantomData , } } } ; }
};
}
