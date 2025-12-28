macro_rules! deps {
    () => {
        AttributeExt!();
    };
}

macro_rules! first_attr_value_str_by_name {
    () => {
        deps!();
        pub fn first_attr_value_str_by_name (attrs : & [impl AttributeExt] , name : Symbol) -> Option < Symbol > { find_by_name (attrs , name) . and_then (| attr | attr . value_str ()) }
    };
}

first_attr_value_str_by_name!()