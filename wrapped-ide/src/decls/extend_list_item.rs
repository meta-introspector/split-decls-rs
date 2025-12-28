macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! extend_list_item {
    () => {
        deps!();
        # [doc = " Extend list item selection to include nearby delimiter and whitespace."] fn extend_list_item (node : & SyntaxNode) -> Option < TextRange > { fn is_single_line_ws (node : & SyntaxToken) -> bool { node . kind () == WHITESPACE && ! node . text () . contains ('\n') } fn nearby_delimiter (delimiter_kind : SyntaxKind , node : & SyntaxNode , dir : Direction ,) -> Option < SyntaxToken > { node . siblings_with_tokens (dir) . skip (1) . find (| node | match node { NodeOrToken :: Node (_) => true , NodeOrToken :: Token (it) => ! is_single_line_ws (it) , }) . and_then (| it | it . into_token ()) . filter (| node | node . kind () == delimiter_kind) } let delimiter = match node . kind () { TYPE_BOUND => T ! [+] , _ => T ! [,] , } ; if let Some (delimiter_node) = nearby_delimiter (delimiter , node , Direction :: Next) { let final_node = delimiter_node . next_sibling_or_token () . and_then (| it | it . into_token ()) . filter (is_single_line_ws) . unwrap_or (delimiter_node) ; return Some (TextRange :: new (node . text_range () . start () , final_node . text_range () . end ())) ; } if let Some (delimiter_node) = nearby_delimiter (delimiter , node , Direction :: Prev) { return Some (TextRange :: new (delimiter_node . text_range () . start () , node . text_range () . end ())) ; } None }
    };
}

extend_list_item!();