macro_rules! char_get {
    () => {
        fn char_get (s : & str , i : usize) -> Option < char > { s [i ..] . chars () . next () }
    };
}

char_get!();