macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! swap_sibling_in_list {
    () => {
        deps!();
        fn swap_sibling_in_list < A : AstNode + Clone , I : Iterator < Item = A > > (node : & SyntaxNode , list : I , range : TextRange , direction : Direction ,) -> Option < TextEdit > { let list_lookup = list . tuple_windows () . find (| (l , r) | match direction { Direction :: Up => r . syntax () . text_range () . contains_range (range) , Direction :: Down => l . syntax () . text_range () . contains_range (range) , }) ; if let Some ((l , r)) = list_lookup { Some (replace_nodes (range , l . syntax () , r . syntax ())) } else { find_ancestors (SyntaxElement :: Node (node . parent () ?) , direction , range) } }
    };
}

swap_sibling_in_list!();