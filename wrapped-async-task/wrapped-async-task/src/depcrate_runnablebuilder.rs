// Generated macro for Builder (struct)
macro_rules! Depcrate_runnableBuilder {
() => {
// Module: crate::runnable
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder that creates a new task."] # [derive (Debug)] pub struct Builder < M > { # [doc = " The metadata associated with the task."] pub (crate) metadata : M , # [doc = " Whether or not a panic that occurs in the task should be propagated."] # [cfg (feature = "std")] pub (crate) propagate_panic : bool , }
};
}
