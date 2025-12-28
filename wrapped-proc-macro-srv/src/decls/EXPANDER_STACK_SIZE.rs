macro_rules! EXPANDER_STACK_SIZE {
    () => {
        const EXPANDER_STACK_SIZE : usize = 8 * 1024 * 1024 ;
    };
}

EXPANDER_STACK_SIZE!();