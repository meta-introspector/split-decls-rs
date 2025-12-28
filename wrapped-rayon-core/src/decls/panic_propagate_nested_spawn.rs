macro_rules! panic_propagate_nested_spawn {
    () => {
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_nested_spawn () { scope (| s | s . spawn (| s | s . spawn (| s | s . spawn (| _ | panic ! ("Hello, world!"))))) ; }
    };
}

panic_propagate_nested_spawn!()