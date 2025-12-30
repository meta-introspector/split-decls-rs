// Generated macro for JobRef (struct)
macro_rules! Depcrate_jobJobRef {
() => {
// Module: crate::job
// Provides: {"JobRef"}
// Dependencies: {}
# [doc = " Effectively a Job trait object. Each JobRef **must** be executed"] # [doc = " exactly once, or else data may leak."] # [doc = ""] # [doc = " Internally, we store the job's data in a `*const ()` pointer.  The"] # [doc = " true type is something like `*const StackJob<...>`, but we hide"] # [doc = " it. We also carry the \"execute fn\" from the `Job` trait."] pub (super) struct JobRef { pointer : * const () , execute_fn : unsafe fn (* const ()) , }
};
}
