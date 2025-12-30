// Generated macro for AWAITER (const)
macro_rules! Depcrate_stateAWAITER {
() => {
// Module: crate::state
// Provides: {"AWAITER"}
// Dependencies: {}
# [doc = " Set if the `Task` is awaiting the output."] # [doc = ""] # [doc = " This flag is set while there is a registered awaiter of type `Waker` inside the task. When the"] # [doc = " task gets closed or completed, we need to wake the awaiter. This flag can be used as a fast"] # [doc = " check that tells us if we need to wake anyone."] pub (crate) const AWAITER : usize = 1 << 5 ;
};
}
