macro_rules! walk_flat_tree_prefix {
    () => {
        # [test] fn walk_flat_tree_prefix () { let v : Vec < _ > = crate :: iter :: walk_tree_prefix (0 , | & e | if e < 99 { Some (e + 1) } else { None }) . collect () ; assert ! (v . into_iter () . eq (0 .. 100)) ; }
    };
}

walk_flat_tree_prefix!()