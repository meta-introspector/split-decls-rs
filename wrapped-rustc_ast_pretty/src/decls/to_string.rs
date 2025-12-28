macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! to_string {
    () => {
        deps!();
        pub fn to_string (f : impl FnOnce (& mut State < '_ >)) -> String { State :: to_string (f) }
    };
}

to_string!()