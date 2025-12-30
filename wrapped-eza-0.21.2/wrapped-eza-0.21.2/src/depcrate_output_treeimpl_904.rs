// Generated macro for impl_904 (impl)
macro_rules! Depcrate_output_treeimpl_904 {
() => {
// Module: crate::output::tree
// Provides: {"impl_904"}
// Dependencies: {}
impl TreeTrunk { # [doc = " Calculates the tree parts for an entry at the given depth and"] # [doc = " last-ness. The depth is used to determine where in the stack the tree"] # [doc = " part should be inserted, and the last-ness is used to determine which"] # [doc = " type of tree part to insert."] # [doc = ""] # [doc = " This takes a `&mut self` because the results of each file are stored"] # [doc = " and used in future rows."] pub fn new_row (& mut self , params : TreeParams) -> & [TreePart] { if let Some (last) = self . last_params { self . stack [last . depth . 0] = if last . last { TreePart :: Blank } else { TreePart :: Line } ; } self . stack . resize (params . depth . 0 + 1 , TreePart :: Edge) ; self . stack [params . depth . 0] = if params . last { TreePart :: Corner } else { TreePart :: Edge } ; self . last_params = Some (params) ; & self . stack [1 ..] } }
};
}
