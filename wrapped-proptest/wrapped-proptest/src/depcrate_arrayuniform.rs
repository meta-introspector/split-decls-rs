// Generated macro for uniform (function)
macro_rules! Depcrate_arrayuniform {
() => {
// Module: crate::array
// Provides: {"uniform"}
// Dependencies: {}
# [doc = " Create a strategy to generate fixed-length arrays."] # [doc = ""] # [doc = " All values within the new strategy are generated using the given"] # [doc = " strategy."] # [doc = ""] # [doc = " See [`UniformArrayStrategy`](struct.UniformArrayStrategy.html) for"] # [doc = " example usage."] pub fn uniform < S : Strategy , const N : usize > (strategy : S ,) -> UniformArrayStrategy < S , [S :: Value ; N] > { UniformArrayStrategy { strategy , _marker : PhantomData , } }
};
}
