macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! path_to_string {
    () => {
        deps!();
        pub fn path_to_string (p : & ast :: Path) -> String { State :: new () . path_to_string (p) }
    };
}

path_to_string!();