macro_rules! deps {
    () => {
        Entry!();
        Entry128!();
    };
}

macro_rules! test_entry_sizes {
    () => {
        deps!();
        # [test] fn test_entry_sizes () { assert_eq ! (mem :: size_of ::< Entry > () , 64) ; assert_eq ! (mem :: size_of ::< Entry128 > () , 128) ; }
    };
}

test_entry_sizes!()