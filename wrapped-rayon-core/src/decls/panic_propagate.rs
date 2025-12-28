macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! panic_propagate {
    () => {
        deps!();
        # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate () { let thread_pool = ThreadPoolBuilder :: new () . build () . unwrap () ; thread_pool . install (| | { panic ! ("Hello, world!") ; }) ; }
    };
}

panic_propagate!()