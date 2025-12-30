// Generated macro for ConnectionRef (struct)
macro_rules! Depcrate_vtabConnectionRef {
() => {
// Module: crate::vtab
// Provides: {"ConnectionRef"}
// Dependencies: {}
# [doc = " A reference to a connection handle with a lifetime bound to context."] pub struct ConnectionRef < 'ctx > { conn : Connection , phantom : PhantomData < & 'ctx Context > , }
};
}
