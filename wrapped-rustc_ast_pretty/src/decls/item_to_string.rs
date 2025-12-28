macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! item_to_string {
    () => {
        deps!();
        pub fn item_to_string (i : & ast :: Item) -> String { State :: new () . item_to_string (i) }
    };
}

item_to_string!()