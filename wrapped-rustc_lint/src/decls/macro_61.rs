macro_rules! macro_61 {
    () => {
        declare_lint_pass ! (# [doc = " Lint for items marked `pub` that aren't reachable from other crates."] UnreachablePub => [UNREACHABLE_PUB]) ;
    };
}

macro_61!();