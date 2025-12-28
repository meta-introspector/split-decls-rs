macro_rules! deps {
    () => {
        Field!();
        ServerError!();
        ServerResult!();
    };
}

macro_rules! check_max_directives {
    () => {
        deps!();
        fn check_max_directives (doc : & ExecutableDocument , max_directives : usize) -> ServerResult < () > { fn check_selection_set (doc : & ExecutableDocument , selection_set : & Positioned < SelectionSet > , limit_directives : usize ,) -> ServerResult < () > { for selection in & selection_set . node . items { match & selection . node { Selection :: Field (field) => { if field . node . directives . len () > limit_directives { return Err (ServerError :: new (format ! ("The number of directives on the field `{}` cannot be greater than `{}`" , field . node . name . node , limit_directives) , Some (field . pos) ,)) ; } check_selection_set (doc , & field . node . selection_set , limit_directives) ? ; } Selection :: FragmentSpread (fragment_spread) => { if let Some (fragment) = doc . fragments . get (& fragment_spread . node . fragment_name . node) { check_selection_set (doc , & fragment . node . selection_set , limit_directives) ? ; } } Selection :: InlineFragment (inline_fragment) => { check_selection_set (doc , & inline_fragment . node . selection_set , limit_directives ,) ? ; } } } Ok (()) } for (_ , operation) in doc . operations . iter () { check_selection_set (doc , & operation . node . selection_set , max_directives) ? ; } Ok (()) }
    };
}

check_max_directives!();