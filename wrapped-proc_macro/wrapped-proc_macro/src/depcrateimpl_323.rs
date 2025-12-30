// Generated macro for impl_323 (impl)
macro_rules! Depcrateimpl_323 {
() => {
// Module: crate
// Provides: {"impl_323"}
// Dependencies: {}
impl ConcatTreesHelper { fn new (capacity : usize) -> Self { ConcatTreesHelper { trees : Vec :: with_capacity (capacity) } } fn push (& mut self , tree : TokenTree) { self . trees . push (tree_to_bridge_tree (tree)) ; } fn build (self) -> TokenStream { if self . trees . is_empty () { TokenStream (None) } else { TokenStream (Some (bridge :: client :: TokenStream :: concat_trees (None , self . trees))) } } fn append_to (self , stream : & mut TokenStream) { if self . trees . is_empty () { return ; } stream . 0 = Some (bridge :: client :: TokenStream :: concat_trees (stream . 0 . take () , self . trees)) } }
};
}
