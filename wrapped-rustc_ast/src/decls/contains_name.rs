macro_rules! deps {
    () => {
        AttributeExt!();
    };
}

macro_rules! contains_name {
    () => {
        deps!();
        pub fn contains_name (attrs : & [impl AttributeExt] , name : Symbol) -> bool { find_by_name (attrs , name) . is_some () }
    };
}

contains_name!()