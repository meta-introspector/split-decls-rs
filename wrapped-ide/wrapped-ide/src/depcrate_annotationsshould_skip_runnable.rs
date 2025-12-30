// Generated macro for should_skip_runnable (function)
macro_rules! Depcrate_annotationsshould_skip_runnable {
() => {
// Module: crate::annotations
// Provides: {"should_skip_runnable"}
// Dependencies: {}
fn should_skip_runnable (kind : & RunnableKind , binary_target : bool) -> bool { match kind { RunnableKind :: Bin => ! binary_target , _ => false , } }
};
}
