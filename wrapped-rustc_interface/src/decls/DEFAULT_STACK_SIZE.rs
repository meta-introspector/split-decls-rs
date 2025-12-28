macro_rules! DEFAULT_STACK_SIZE {
    () => {
        pub const DEFAULT_STACK_SIZE : usize = 8 * 1024 * 1024 ;
    };
}

DEFAULT_STACK_SIZE!();