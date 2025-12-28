macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! recursion_depth {
    () => {
        deps!();
        macro_rules ! recursion_depth { ($ remaining_depth : ident) => { { if $ remaining_depth == 0 { return Err (Error :: RecursionLimitExceeded) ; } $ remaining_depth - 1 } } ; }
    };
}

recursion_depth!()