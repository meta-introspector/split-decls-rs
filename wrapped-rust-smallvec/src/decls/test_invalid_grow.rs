macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_invalid_grow {
    () => {
        deps!();
        # [test] # [should_panic] fn test_invalid_grow () { let mut v : SmallVec < u8 , 8 > = SmallVec :: new () ; v . extend (0 .. 8) ; v . grow (5) ; }
    };
}

test_invalid_grow!()