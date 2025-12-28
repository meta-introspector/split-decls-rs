macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! vis_to_string {
    () => {
        deps!();
        pub fn vis_to_string (v : & ast :: Visibility) -> String { State :: new () . vis_to_string (v) }
    };
}

vis_to_string!();