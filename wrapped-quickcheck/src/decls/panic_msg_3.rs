macro_rules! panic_msg_3 {
    () => {
        # [test] # [should_panic (expected = "foo")] fn panic_msg_3 () { fn prop () -> bool { assert_eq ! ("foo" , "bar") ; true } quickcheck (prop as fn () -> bool) ; }
    };
}

panic_msg_3!()