// Generated macro for impl_906 (impl)
macro_rules! Depcrate_output_treeimpl_906 {
() => {
// Module: crate::output::tree
// Provides: {"impl_906"}
// Dependencies: {}
impl TreeDepth { pub fn root () -> Self { Self (0) } pub fn deeper (self) -> Self { Self (self . 0 + 1) } # [doc = " Creates an iterator that, as well as yielding each value, yields a"] # [doc = " `TreeParams` with the current depth and last flag filled in."] pub fn iterate_over < I , T > (self , inner : I) -> Iter < I > where I : ExactSizeIterator + Iterator < Item = T > , { Iter { current_depth : self , inner , } } }
};
}
