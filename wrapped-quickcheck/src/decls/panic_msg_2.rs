macro_rules! panic_msg_2 {
    () => {
        # [test] # [should_panic (expected = "foo")] fn panic_msg_2 () { fn prop () -> bool { assert ! ("foo" == "bar") ; true } quickcheck (prop as fn () -> bool) ; }
    };
}

panic_msg_2!();