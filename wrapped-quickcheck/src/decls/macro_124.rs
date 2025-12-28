macro_rules! macro_124 {
    () => {
        quickcheck ! { # [doc = " The following is a very simplistic test, which only verifies"] # [doc = " that our PathBuf::arbitrary does not panic.  Still, that's"] # [doc = " something!  :)"] fn pathbuf (_p : PathBuf) -> bool { true } fn basic_hashset (_set : HashSet < u8 >) -> bool { true } fn basic_hashmap (_map : HashMap < u8 , u8 >) -> bool { true } fn substitute_hashset (_set : HashSet < u8 , BuildHasherDefault < DefaultHasher >>) -> bool { true } fn substitute_hashmap (_map : HashMap < u8 , u8 , BuildHasherDefault < DefaultHasher >>) -> bool { true } fn cstring (_p : CString) -> bool { true } }
    };
}

macro_124!()