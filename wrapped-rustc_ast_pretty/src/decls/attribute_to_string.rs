macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! attribute_to_string {
    () => {
        deps!();
        pub fn attribute_to_string (attr : & ast :: Attribute) -> String { State :: new () . attribute_to_string (attr) }
    };
}

attribute_to_string!();