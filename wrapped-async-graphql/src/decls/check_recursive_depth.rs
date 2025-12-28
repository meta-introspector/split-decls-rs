macro_rules! deps {
    () => {
        ServerResult!();
        Field!();
        ServerError!();
    };
}

macro_rules! check_recursive_depth {
    () => {
        deps!();
        fn check_recursive_depth (doc : & ExecutableDocument , max_depth : usize) -> ServerResult < () > { fn check_selection_set (doc : & ExecutableDocument , selection_set : & Positioned < SelectionSet > , current_depth : usize , max_depth : usize ,) -> ServerResult < () > { if current_depth > max_depth { return Err (ServerError :: new (format ! ("The recursion depth of the query cannot be greater than `{}`" , max_depth) , Some (selection_set . pos) ,)) ; } for selection in & selection_set . node . items { match & selection . node { Selection :: Field (field) => { if ! field . node . selection_set . node . items . is_empty () { check_selection_set (doc , & field . node . selection_set , current_depth + 1 , max_depth ,) ? ; } } Selection :: FragmentSpread (fragment_spread) => { if let Some (fragment) = doc . fragments . get (& fragment_spread . node . fragment_name . node) { check_selection_set (doc , & fragment . node . selection_set , current_depth + 1 , max_depth ,) ? ; } } Selection :: InlineFragment (inline_fragment) => { check_selection_set (doc , & inline_fragment . node . selection_set , current_depth + 1 , max_depth ,) ? ; } } } Ok (()) } for (_ , operation) in doc . operations . iter () { check_selection_set (doc , & operation . node . selection_set , 0 , max_depth) ? ; } Ok (()) }
    };
}

check_recursive_depth!();