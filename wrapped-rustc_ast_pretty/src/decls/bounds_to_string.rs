macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! bounds_to_string {
    () => {
        deps!();
        pub fn bounds_to_string (bounds : & [ast :: GenericBound]) -> String { State :: new () . bounds_to_string (bounds) }
    };
}

bounds_to_string!()