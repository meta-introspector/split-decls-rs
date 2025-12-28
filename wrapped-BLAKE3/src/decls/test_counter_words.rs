macro_rules! test_counter_words {
    () => {
        # [test] fn test_counter_words () { let counter : u64 = (1 << 32) + 2 ; assert_eq ! (crate :: counter_low (counter) , 2) ; assert_eq ! (crate :: counter_high (counter) , 1) ; }
    };
}

test_counter_words!();