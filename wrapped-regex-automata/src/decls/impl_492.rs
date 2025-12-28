macro_rules! deps {
    () => {
        Frame!();
        State!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl < 'a > Frame < 'a > { # [doc = " Create a new stack frame for trie traversal. This initializes the"] # [doc = " 'transitions' iterator to the transitions for the first chunk, with the"] # [doc = " 'chunks' iterator being every chunk after the first one."] fn new (state : & 'a State) -> Frame < 'a > { let mut chunks = state . chunks () ; let chunk = chunks . next () . unwrap () ; let transitions = chunk . iter () ; Frame { chunks , transitions , union : vec ! [] , sparse : vec ! [] } } }
    };
}

impl_492!();