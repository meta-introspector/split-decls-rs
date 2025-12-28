macro_rules! walk_flat_tree_postfix {
    () => {
        # [test] fn walk_flat_tree_postfix () { let v : Vec < _ > = crate :: iter :: walk_tree_postfix (99 , | & e | if e > 0 { Some (e - 1) } else { None }) . collect () ; assert ! (v . into_iter () . eq (0 .. 100)) ; }
    };
}

walk_flat_tree_postfix!();