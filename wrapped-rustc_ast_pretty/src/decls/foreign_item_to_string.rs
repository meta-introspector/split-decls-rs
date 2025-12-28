macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! foreign_item_to_string {
    () => {
        deps!();
        pub fn foreign_item_to_string (i : & ast :: ForeignItem) -> String { State :: new () . foreign_item_to_string (i) }
    };
}

foreign_item_to_string!();