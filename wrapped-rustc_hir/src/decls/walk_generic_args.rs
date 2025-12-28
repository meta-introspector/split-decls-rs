macro_rules! deps {
    () => {
        GenericArgs!();
        Visitor!();
    };
}

macro_rules! walk_generic_args {
    () => {
        deps!();
        pub fn walk_generic_args < 'v , V : Visitor < 'v > > (visitor : & mut V , generic_args : & 'v GenericArgs < 'v > ,) -> V :: Result { let GenericArgs { args , constraints , parenthesized : _ , span_ext : _ } = generic_args ; walk_list ! (visitor , visit_generic_arg , * args) ; walk_list ! (visitor , visit_assoc_item_constraint , * constraints) ; V :: Result :: output () }
    };
}

walk_generic_args!()