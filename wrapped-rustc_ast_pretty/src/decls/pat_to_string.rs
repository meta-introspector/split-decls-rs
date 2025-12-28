macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! pat_to_string {
    () => {
        deps!();
        pub fn pat_to_string (pat : & ast :: Pat) -> String { State :: new () . pat_to_string (pat) }
    };
}

pat_to_string!();