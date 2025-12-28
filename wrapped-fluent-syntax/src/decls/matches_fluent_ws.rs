macro_rules! matches_fluent_ws {
    () => {
        pub (crate) fn matches_fluent_ws (c : char) -> bool { c == ' ' || c == '\r' || c == '\n' }
    };
}

matches_fluent_ws!()