macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! contains_lines {
    () => {
        deps!();
        pub fn contains_lines (hunks : & [Hunk]) -> bool { hunks . iter () . any (| h | ! h . after . is_empty ()) }
    };
}

contains_lines!()