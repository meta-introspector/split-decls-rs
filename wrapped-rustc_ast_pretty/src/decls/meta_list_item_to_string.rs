macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! meta_list_item_to_string {
    () => {
        deps!();
        pub fn meta_list_item_to_string (li : & ast :: MetaItemInner) -> String { State :: new () . meta_list_item_to_string (li) }
    };
}

meta_list_item_to_string!();