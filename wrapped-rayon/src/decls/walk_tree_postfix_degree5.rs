macro_rules! walk_tree_postfix_degree5 {
    () => {
        # [test] fn walk_tree_postfix_degree5 () { let depth = 5 ; let nodes_number = (1 - 5i32 . pow (depth)) / (1 - 5) ; let nodes = (0 .. nodes_number) . collect :: < Vec < _ > > () ; let v : Vec < i32 > = crate :: iter :: walk_tree_postfix (nodes . as_slice () , | & r | { r . split_last () . into_iter () . filter_map (| (_ , r) | if r . is_empty () { None } else { Some (r) }) . flat_map (| r | r . chunks (r . len () / 5)) }) . filter_map (| r | r . last () . copied ()) . collect () ; assert_eq ! (v , nodes) }
    };
}

walk_tree_postfix_degree5!();