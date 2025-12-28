macro_rules! ArrayGuard {
    () => {
        # [doc = " Helper to safely create arrays since the standard library doesn't"] # [doc = " provide one yet. Shouldn't be necessary in the future."] struct ArrayGuard < T , const N : usize > { dst : * mut T , initialized : usize , }
    };
}

ArrayGuard!()