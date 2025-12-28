macro_rules! MAX_UNWIND_STACK_DEPTH {
    () => {
        # [cfg (feature = "read")] const MAX_UNWIND_STACK_DEPTH : usize = 4 ;
    };
}

MAX_UNWIND_STACK_DEPTH!()