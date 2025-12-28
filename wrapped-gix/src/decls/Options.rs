macro_rules! deps {
    () => {
        Sorting!();
        Note!();
        Default!();
        Clone!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " Options for use with [Repository::index_worktree_status()]."] # [derive (Default , Debug , Clone , Copy , PartialEq)] pub struct Options { # [doc = " The way all output should be sorted."] # [doc = ""] # [doc = " If `None`, and depending on the `rewrites` field, output will be immediate but the output order"] # [doc = " isn't determined, and may differ between two runs. `rewrites` also depend on the order of entries that"] # [doc = " are presented to it, hence for deterministic results, sorting needs to be enabled."] # [doc = ""] # [doc = " If `Some(_)`, all entries are collected beforehand, so they can be sorted before outputting any of them"] # [doc = " to the user."] # [doc = ""] # [doc = " If immediate output of entries in any order is desired, this should be `None`,"] # [doc = " along with `rewrites` being `None` as well."] pub sorting : Option < gix_status :: index_as_worktree_with_renames :: Sorting > , # [doc = " If not `None`, the options to configure the directory walk, determining how its results will look like."] # [doc = ""] # [doc = " If `None`, only modification checks are performed."] # [doc = ""] # [doc = " Can be instantiated with [Repository::dirwalk_options()]."] pub dirwalk_options : Option < crate :: dirwalk :: Options > , # [doc = " If `Some(_)`, along with `Some(_)` in `dirwalk_options`, rewrite tracking will be performed between the"] # [doc = " index and the working tree."] # [doc = " Note that there is no git-configuration specific to index-worktree rename tracking."] # [doc = " When rewrite tracking is enabled, there will be a delay for some entries as they partake in the rename-analysis."] pub rewrites : Option < gix_diff :: Rewrites > , # [doc = " If set, don't use more than this amount of threads for the tracked modification check."] # [doc = " Otherwise, usually use as many threads as there are logical cores."] # [doc = " A value of 0 is interpreted as no-limit"] pub thread_limit : Option < usize > , }
    };
}

Options!()