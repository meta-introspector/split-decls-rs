// Generated macro for UnpinImpl (enum)
macro_rules! Depcrate_pin_project_argsUnpinImpl {
() => {
// Module: crate::pin_project::args
// Provides: {"UnpinImpl"}
// Dependencies: {}
# [doc = " `UnsafeUnpin` or `!Unpin` argument."] # [derive (Clone , Copy)] pub (super) enum UnpinImpl { Default , # [doc = " `UnsafeUnpin`."] Unsafe (Span) , # [doc = " `!Unpin`."] Negative (Span) , }
};
}
