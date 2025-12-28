macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! tt_to_string {
    () => {
        deps!();
        pub fn tt_to_string (tt : & TokenTree) -> String { State :: new () . tt_to_string (tt) }
    };
}

tt_to_string!()