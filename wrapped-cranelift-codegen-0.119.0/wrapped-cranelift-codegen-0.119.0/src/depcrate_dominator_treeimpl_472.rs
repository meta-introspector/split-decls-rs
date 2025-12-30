// Generated macro for impl_472 (impl)
macro_rules! Depcrate_dominator_treeimpl_472 {
() => {
// Module: crate::dominator_tree
// Provides: {"impl_472"}
// Dependencies: {}
impl SpanningTree { fn new () -> Self { Self { nodes : vec ! [Default :: default ()] , } } fn with_capacity (capacity : usize) -> Self { let mut nodes = Vec :: with_capacity (capacity + 1) ; nodes . push (Default :: default ()) ; Self { nodes } } fn len (& self) -> usize { self . nodes . len () } fn reserve (& mut self , capacity : usize) { self . nodes . reserve (capacity) ; } fn clear (& mut self) { self . nodes . resize (1 , Default :: default ()) ; } # [doc = " Returns pre_number for the new node."] fn push (& mut self , ancestor : u32 , block : Block) -> u32 { debug_assert ! (! self . nodes . is_empty ()) ; let pre_number = self . nodes . len () as u32 ; self . nodes . push (SpanningTreeNode { block : block . into () , ancestor : ancestor , label : pre_number , semi : pre_number , idom : ancestor , }) ; pre_number } }
};
}
