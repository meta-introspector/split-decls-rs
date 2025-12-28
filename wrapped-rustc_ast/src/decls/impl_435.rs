macro_rules! deps {
    () => {
        FlatToken!();
        Token!();
        AttrTokenStream!();
        LazyAttrTokenStreamInner!();
        AttrsTarget!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl LazyAttrTokenStreamInner { fn to_attr_token_stream (& self) -> AttrTokenStream { match self { LazyAttrTokenStreamInner :: Direct (stream) => stream . clone () , LazyAttrTokenStreamInner :: Pending { start_token , cursor_snapshot , num_calls , break_last_token , node_replacements , } => { let mut cursor_snapshot = cursor_snapshot . clone () ; let tokens = iter :: once (FlatToken :: Token (* start_token)) . chain (iter :: repeat_with (| | FlatToken :: Token (cursor_snapshot . next ()))) . take (* num_calls as usize) ; if node_replacements . is_empty () { make_attr_token_stream (tokens , * break_last_token) } else { let mut tokens : Vec < _ > = tokens . collect () ; let mut node_replacements = node_replacements . to_vec () ; node_replacements . sort_by_key (| (range , _) | range . 0 . start) ; # [cfg (debug_assertions)] for [(node_range , tokens) , (next_node_range , next_tokens)] in node_replacements . array_windows () { assert ! (node_range . 0 . end <= next_node_range . 0 . start || node_range . 0 . end >= next_node_range . 0 . end , "Node ranges should be disjoint or nested: ({:?}, {:?}) ({:?}, {:?})" , node_range , tokens , next_node_range , next_tokens ,) ; } for (node_range , target) in node_replacements . into_iter () . rev () { assert ! (! node_range . 0 . is_empty () , "Cannot replace an empty node range: {:?}" , node_range . 0) ; let target_len = target . is_some () as usize ; tokens . splice ((node_range . 0 . start as usize) .. (node_range . 0 . end as usize) , target . into_iter () . map (| target | FlatToken :: AttrsTarget (target)) . chain (iter :: repeat (FlatToken :: Empty) . take (node_range . 0 . len () - target_len) ,) ,) ; } make_attr_token_stream (tokens . into_iter () , * break_last_token) } } } } }
    };
}

impl_435!()