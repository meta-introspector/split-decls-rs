macro_rules! CALL_STACK_INITIAL_CAPACITY {
    () => {
        # [doc = " Number of call stacks that may result from a sequence of rules parsing."] const CALL_STACK_INITIAL_CAPACITY : usize = 20 ;
    };
}

CALL_STACK_INITIAL_CAPACITY!();