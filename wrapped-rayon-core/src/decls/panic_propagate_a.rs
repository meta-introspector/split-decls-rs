macro_rules! panic_propagate_a {
    () => {
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_a () { join (| | panic ! ("Hello, world!") , | | ()) ; }
    };
}

panic_propagate_a!();