macro_rules! panic_propagate_both {
    () => {
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_both () { join (| | panic ! ("Hello, world!") , | | panic ! ("Goodbye, world!")) ; }
    };
}

panic_propagate_both!();