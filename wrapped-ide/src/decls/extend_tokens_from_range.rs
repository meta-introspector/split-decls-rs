macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! extend_tokens_from_range {
    () => {
        deps!();
        fn extend_tokens_from_range (sema : & Semantics < '_ , RootDatabase > , macro_call : ast :: MacroCall , original_range : TextRange ,) -> Option < TextRange > { let src = macro_call . syntax () . covering_element (original_range) ; let (first_token , last_token) = match src { NodeOrToken :: Node (it) => (it . first_token () ? , it . last_token () ?) , NodeOrToken :: Token (it) => (it . clone () , it) , } ; let mut first_token = skip_trivia_token (first_token , Direction :: Next) ? ; let mut last_token = skip_trivia_token (last_token , Direction :: Prev) ? ; while ! original_range . contains_range (first_token . text_range ()) { first_token = skip_trivia_token (first_token . next_token () ? , Direction :: Next) ? ; } while ! original_range . contains_range (last_token . text_range ()) { last_token = skip_trivia_token (last_token . prev_token () ? , Direction :: Prev) ? ; } let extended = { let fst_expanded = sema . descend_into_macros_single_exact (first_token . clone ()) ; let lst_expanded = sema . descend_into_macros_single_exact (last_token . clone ()) ; let mut lca = algo :: least_common_ancestor (& fst_expanded . parent () ? , & lst_expanded . parent () ?) ? ; lca = shallowest_node (& lca) ; if lca . first_token () == Some (fst_expanded) && lca . last_token () == Some (lst_expanded) { lca = lca . parent () ? ; } lca } ; let validate = | | { let extended = & extended ; move | token : & SyntaxToken | -> bool { let expanded = sema . descend_into_macros_single_exact (token . clone ()) ; let parent = match expanded . parent () { Some (it) => it , None => return false , } ; algo :: least_common_ancestor (extended , & parent) . as_ref () == Some (extended) } } ; let first = successors (Some (first_token) , | token | { let token = token . prev_token () ? ; skip_trivia_token (token , Direction :: Prev) }) . take_while (validate ()) . last () ? ; let last = successors (Some (last_token) , | token | { let token = token . next_token () ? ; skip_trivia_token (token , Direction :: Next) }) . take_while (validate ()) . last () ? ; let range = first . text_range () . cover (last . text_range ()) ; if range . contains_range (original_range) && original_range != range { Some (range) } else { None } }
    };
}

extend_tokens_from_range!();