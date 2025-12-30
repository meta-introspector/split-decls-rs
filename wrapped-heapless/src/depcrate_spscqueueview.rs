// Generated macro for QueueView (type)
macro_rules! Depcrate_spscQueueView {
() => {
// Module: crate::spsc
// Provides: {"QueueView"}
// Dependencies: {}
# [doc = " A [`Queue`] with dynamic capacity."] # [doc = ""] # [doc = " [`Queue`] coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by"] # [doc = " reference."] pub type QueueView < T > = QueueInner < T , ViewStorage > ;
};
}
