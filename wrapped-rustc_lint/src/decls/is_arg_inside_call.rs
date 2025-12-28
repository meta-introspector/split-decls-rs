macro_rules! is_arg_inside_call {
    () => {
        fn is_arg_inside_call (arg : Span , call : Span) -> bool { call . contains (arg) && ! call . source_equal (arg) }
    };
}

is_arg_inside_call!();