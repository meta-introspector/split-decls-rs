macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! referenced_variables_to_vec {
    () => {
        deps!();
        fn referenced_variables_to_vec < 'a > (value : & 'a Value , vars : & mut Vec < & 'a str >) { match value { Value :: Variable (name) => { vars . push (name) ; } Value :: List (values) => values . iter () . for_each (| value | referenced_variables_to_vec (value , vars)) , Value :: Object (obj) => obj . values () . for_each (| value | referenced_variables_to_vec (value , vars)) , _ => { } } }
    };
}

referenced_variables_to_vec!();