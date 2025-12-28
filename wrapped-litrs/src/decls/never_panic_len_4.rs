macro_rules! never_panic_len_4 {
    () => {
        # [test] # [ignore] fn never_panic_len_4 () { for a in 0 .. 128 { for b in 0 .. 128 { for c in 0 .. 128 { for d in 0 .. 128 { assert_no_panic ! ([a , b , c , d]) ; } } } } }
    };
}

never_panic_len_4!()