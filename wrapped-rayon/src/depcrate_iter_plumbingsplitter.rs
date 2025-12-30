// Generated macro for Splitter (struct)
macro_rules! Depcrate_iter_plumbingSplitter {
() => {
// Module: crate::iter::plumbing
// Provides: {"Splitter"}
// Dependencies: {}
# [doc = " A splitter controls the policy for splitting into smaller work items."] # [doc = ""] # [doc = " Thief-splitting is an adaptive policy that starts by splitting into"] # [doc = " enough jobs for every worker thread, and then resets itself whenever a"] # [doc = " job is actually stolen into a different thread."] # [derive (Clone , Copy)] struct Splitter { # [doc = " The `splits` tell us approximately how many remaining times we'd"] # [doc = " like to split this job.  We always just divide it by two though, so"] # [doc = " the effective number of pieces will be `next_power_of_two()`."] splits : usize , }
};
}
