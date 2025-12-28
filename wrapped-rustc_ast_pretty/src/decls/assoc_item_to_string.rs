macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! assoc_item_to_string {
    () => {
        deps!();
        pub fn assoc_item_to_string (i : & ast :: AssocItem) -> String { State :: new () . assoc_item_to_string (i) }
    };
}

assoc_item_to_string!();