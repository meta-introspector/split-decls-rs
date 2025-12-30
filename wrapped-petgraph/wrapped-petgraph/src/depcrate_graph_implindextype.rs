// Generated macro for IndexType (trait)
macro_rules! Depcrate_graph_implIndexType {
() => {
// Module: crate::graph_impl
// Provides: {"IndexType"}
// Dependencies: {}
# [doc = " Trait for the unsigned integer type used for node and edge indices."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Marked `unsafe` because: the trait must faithfully preserve"] # [doc = " and convert index values."] pub unsafe trait IndexType : Copy + Default + Hash + Ord + fmt :: Debug + 'static { fn new (x : usize) -> Self ; fn index (& self) -> usize ; fn max () -> Self ; }
};
}
