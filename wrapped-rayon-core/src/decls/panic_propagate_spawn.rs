macro_rules! panic_propagate_spawn {
    () => {
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_spawn () { scope (| s | s . spawn (| _ | panic ! ("Hello, world!"))) ; }
    };
}

panic_propagate_spawn!()