macro_rules! referenced_variables {
    () => {
        pub fn referenced_variables (value : & Value) -> Vec < & str > { let mut vars = Vec :: new () ; referenced_variables_to_vec (value , & mut vars) ; vars }
    };
}

referenced_variables!()