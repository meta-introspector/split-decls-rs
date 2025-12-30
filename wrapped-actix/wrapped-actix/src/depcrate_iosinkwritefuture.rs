// Generated macro for SinkWriteFuture (struct)
macro_rules! Depcrate_ioSinkWriteFuture {
() => {
// Module: crate::io
// Provides: {"SinkWriteFuture"}
// Dependencies: {}
struct SinkWriteFuture < I : 'static , S : Sink < I > > { inner : Rc < RefCell < InnerSinkWrite < I , S > > > , }
};
}
