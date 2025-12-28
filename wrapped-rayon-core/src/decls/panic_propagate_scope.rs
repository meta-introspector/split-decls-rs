macro_rules! panic_propagate_scope {
    () => {
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_scope () { scope (| _ | panic ! ("Hello, world!")) ; }
    };
}

panic_propagate_scope!()