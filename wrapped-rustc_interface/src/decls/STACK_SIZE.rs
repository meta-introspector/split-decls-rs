macro_rules! STACK_SIZE {
    () => {
        pub static STACK_SIZE : OnceLock < usize > = OnceLock :: new () ;
    };
}

STACK_SIZE!();