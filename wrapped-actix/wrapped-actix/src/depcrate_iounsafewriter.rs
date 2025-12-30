// Generated macro for UnsafeWriter (struct)
macro_rules! Depcrate_ioUnsafeWriter {
() => {
// Module: crate::io
// Provides: {"UnsafeWriter"}
// Dependencies: {}
struct UnsafeWriter < T : AsyncWrite , E : From < io :: Error > > (Rc < RefCell < InnerWriter < E > > > , Rc < RefCell < T > >) ;
};
}
