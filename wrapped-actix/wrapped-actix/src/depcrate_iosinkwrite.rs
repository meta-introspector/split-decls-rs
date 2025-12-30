// Generated macro for SinkWrite (struct)
macro_rules! Depcrate_ioSinkWrite {
() => {
// Module: crate::io
// Provides: {"SinkWrite"}
// Dependencies: {}
# [doc = " A wrapper for the `Sink` type."] pub struct SinkWrite < I , S : Sink < I > + Unpin > { inner : Rc < RefCell < InnerSinkWrite < I , S > > > , }
};
}
