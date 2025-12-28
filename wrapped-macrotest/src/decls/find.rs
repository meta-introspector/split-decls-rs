macro_rules! find {
    () => {
        pub fn find () -> Option < Vec < String > > { try_find () . ok () }
    };
}

find!();