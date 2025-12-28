macro_rules! never_panic_up_to_3 {
    () => {
        # [test] # [ignore] fn never_panic_up_to_3 () { for a in 0 .. 128 { assert_no_panic ! ([a]) ; for b in 0 .. 128 { assert_no_panic ! ([a , b]) ; for c in 0 .. 128 { assert_no_panic ! ([a , b , c]) ; } } } }
    };
}

never_panic_up_to_3!()