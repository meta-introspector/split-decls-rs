macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! is_system_type {
    () => {
        deps!();
        fn is_system_type (name : & str) -> bool { if name . starts_with ("__") { return true ; } name == "Boolean" || name == "Int" || name == "Float" || name == "String" || name == "ID" }
    };
}

is_system_type!();