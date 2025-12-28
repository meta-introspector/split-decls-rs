macro_rules! panic_msg_1 {
    () => {
        # [test] # [should_panic (expected = "foo")] fn panic_msg_1 () { fn prop () -> bool { panic ! ("foo") ; } quickcheck (prop as fn () -> bool) ; }
    };
}

panic_msg_1!()