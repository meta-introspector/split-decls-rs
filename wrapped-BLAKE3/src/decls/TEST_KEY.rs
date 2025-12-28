macro_rules! deps {
    () => {
        CVBytes!();
    };
}

macro_rules! TEST_KEY {
    () => {
        deps!();
        pub const TEST_KEY : CVBytes = * b"whats the Elvish word for friend" ;
    };
}

TEST_KEY!();