// Generated macro for UnboundedReceiver (struct)
macro_rules! Depcrate_mpscUnboundedReceiver {
() => {
// Module: crate::mpsc
// Provides: {"UnboundedReceiver"}
// Dependencies: {}
# [doc = " The receiving end of an unbounded mpsc channel."] # [doc = ""] # [doc = " This value is created by the [`unbounded`] function."] pub struct UnboundedReceiver < T > { inner : Option < Arc < UnboundedInner < T > > > , }
};
}
