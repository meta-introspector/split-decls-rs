// Generated macro for ConnectionRef (struct)
macro_rules! Depcrate_functionsConnectionRef {
() => {
// Module: crate::functions
// Provides: {"ConnectionRef"}
// Dependencies: {}
# [doc = " A reference to a connection handle with a lifetime bound to something."] pub struct ConnectionRef < 'ctx > { conn : Connection , phantom : PhantomData < & 'ctx Context < 'ctx > > , }
};
}
