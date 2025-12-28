macro_rules! ClauseFlavor {
    () => {
        enum ClauseFlavor { # [doc = " Predicate comes from `predicates_of`."] Where , # [doc = " Predicate comes from `const_conditions`."] Const , }
    };
}

ClauseFlavor!()