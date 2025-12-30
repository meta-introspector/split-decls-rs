// Generated macro for macro_127 (macro)
macro_rules! Depcrate_streammacro_127 {
() => {
// Module: crate::stream
// Provides: {"macro_127"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::partition()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PartitionFuture < S , P , B > { # [pin] stream : S , predicate : P , res : Option < (B , B) >, } }
};
}
