macro_rules! InlinedFunctionAddress {
    () => {
        pub (crate) struct InlinedFunctionAddress { range : gimli :: Range , call_depth : usize , # [doc = " An index into `Function::inlined_functions`."] function : usize , }
    };
}

InlinedFunctionAddress!();