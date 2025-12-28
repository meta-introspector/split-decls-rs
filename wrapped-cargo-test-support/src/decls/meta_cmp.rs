macro_rules! meta_cmp {
    () => {
        fn meta_cmp (a : & str , mut b : & str) -> bool { for (i , part) in a . split ("[..]") . enumerate () { match b . find (part) { Some (j) => { if i == 0 && j != 0 { return false ; } b = & b [j + part . len () ..] ; } None => return false , } } b . is_empty () || a . ends_with ("[..]") }
    };
}

meta_cmp!()