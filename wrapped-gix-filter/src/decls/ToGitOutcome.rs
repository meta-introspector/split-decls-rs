macro_rules! deps {
    () => {
        Process!();
    };
}

macro_rules! ToGitOutcome {
    () => {
        deps!();
        # [doc = " The result of a conversion with zero or more filters to be stored in git."] pub enum ToGitOutcome < 'pipeline , R > { # [doc = " The original input wasn't changed and the reader is still available for consumption."] Unchanged (R) , # [doc = " An external filter (and only that) was applied and its results *have to be consumed*."] Process (Box < dyn std :: io :: Read + 'pipeline >) , # [doc = " A reference to the result of one or more filters of which one didn't support streaming."] # [doc = ""] # [doc = " This can happen if an `eol`, `working-tree-encoding` or `ident` filter is applied, possibly on top of an external filter."] Buffer (& 'pipeline [u8]) , }
    };
}

ToGitOutcome!()