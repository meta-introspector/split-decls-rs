macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! where_bound_predicate_to_string {
    () => {
        deps!();
        pub fn where_bound_predicate_to_string (where_bound_predicate : & ast :: WhereBoundPredicate) -> String { State :: new () . where_bound_predicate_to_string (where_bound_predicate) }
    };
}

where_bound_predicate_to_string!();