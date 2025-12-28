macro_rules! deps {
    () => {
        LazyFunction!();
        FunctionAddress!();
    };
}

macro_rules! Functions {
    () => {
        deps!();
        pub (crate) struct Functions < R : gimli :: Reader > { # [doc = " List of all `DW_TAG_subprogram` details in the unit."] pub (crate) functions : Box < [LazyFunction < R >] > , # [doc = " List of `DW_TAG_subprogram` address ranges in the unit."] pub (crate) addresses : Box < [FunctionAddress] > , }
    };
}

Functions!()