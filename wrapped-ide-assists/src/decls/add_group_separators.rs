macro_rules! add_group_separators {
    () => {
        pub (crate) fn add_group_separators (s : & str , group_size : usize) -> String { let mut chars = Vec :: new () ; for (i , ch) in s . chars () . filter (| & ch | ch != '_') . rev () . enumerate () { if i > 0 && i % group_size == 0 && ch != '-' { chars . push ('_') ; } chars . push (ch) ; } chars . into_iter () . rev () . collect () }
    };
}

add_group_separators!()