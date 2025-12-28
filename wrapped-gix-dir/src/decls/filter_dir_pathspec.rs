macro_rules! deps {
    () => {
        PathspecMatch!();
    };
}

macro_rules! filter_dir_pathspec {
    () => {
        deps!();
        fn filter_dir_pathspec (current : Option < PathspecMatch >) -> Option < PathspecMatch > { current . filter (| m | { matches ! (m , PathspecMatch :: Always | PathspecMatch :: WildcardMatch | PathspecMatch :: Verbatim) }) }
    };
}

filter_dir_pathspec!();