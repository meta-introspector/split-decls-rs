// Generated macro for swap_sibling_in_list (function)
macro_rules! Depcrate_move_itemswap_sibling_in_list {
() => {
// Module: crate::move_item
// Provides: {"swap_sibling_in_list"}
// Dependencies: {}
fn swap_sibling_in_list < A : AstNode + Clone , I : Iterator < Item = A > > (node : & SyntaxNode , list : I , range : TextRange , direction : Direction ,) -> Option < TextEdit > { let list_lookup = list . tuple_windows () . find (| (l , r) | match direction { Direction :: Up => r . syntax () . text_range () . contains_range (range) , Direction :: Down => l . syntax () . text_range () . contains_range (range) , }) ; if let Some ((l , r)) = list_lookup { Some (replace_nodes (range , l . syntax () , r . syntax ())) } else { find_ancestors (SyntaxElement :: Node (node . parent () ?) , direction , range) } }
};
}
