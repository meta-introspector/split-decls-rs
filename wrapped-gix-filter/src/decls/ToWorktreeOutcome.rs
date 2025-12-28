macro_rules! deps {
    () => {
        MaybeDelayed!();
        Process!();
    };
}

macro_rules! ToWorktreeOutcome {
    () => {
        deps!();
        # [doc = " The result of a conversion with zero or more filters."] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " If `std::io::Read` is used on it and the output is delayed, a panic will occur. The caller is responsible for either disallowing delayed"] # [doc = " results or if allowed, handle them. Use [`is_delayed()][Self::is_delayed()]."] pub enum ToWorktreeOutcome < 'input , 'pipeline > { # [doc = " The original input wasn't changed and the original buffer is present"] Unchanged (& 'input [u8]) , # [doc = " A reference to the result of one or more filters of which one didn't support streaming."] # [doc = ""] # [doc = " This can happen if an `eol`, `working-tree-encoding` or `ident` filter is applied, possibly on top of an external filter."] Buffer (& 'pipeline [u8]) , # [doc = " An external filter (and only that) was applied and its results *have to be consumed*. Note that the output might be delayed,"] # [doc = " which requires special handling to eventually receive it."] Process (driver :: apply :: MaybeDelayed < 'pipeline >) , }
    };
}

ToWorktreeOutcome!()