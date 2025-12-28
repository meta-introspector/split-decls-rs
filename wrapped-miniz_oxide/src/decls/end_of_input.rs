macro_rules! deps {
    () => {
        TINFLStatus!();
        Action!();
    };
}

macro_rules! end_of_input {
    () => {
        deps!();
        # [inline] const fn end_of_input (flags : u32) -> Action { Action :: End (if flags & TINFL_FLAG_HAS_MORE_INPUT != 0 { TINFLStatus :: NeedsMoreInput } else { TINFLStatus :: FailedCannotMakeProgress }) }
    };
}

end_of_input!()