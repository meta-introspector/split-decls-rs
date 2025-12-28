macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! crate_to_string_for_macros {
    () => {
        deps!();
        pub fn crate_to_string_for_macros (krate : & ast :: Crate) -> String { State :: to_string (| s | { s . print_inner_attributes (& krate . attrs) ; for item in & krate . items { s . print_item (item) ; } }) }
    };
}

crate_to_string_for_macros!()