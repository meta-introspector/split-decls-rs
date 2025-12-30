// Generated macro for writable_at (function)
macro_rules! Depcratewritable_at {
() => {
// Module: crate
// Provides: {"writable_at"}
// Dependencies: {}
# [doc = " A shortcut to [`Handle::<Writable>::at()`] providing a writable temporary file at the given path."] pub fn writable_at (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove ,) -> io :: Result < Handle < Writable > > { Handle :: < Writable > :: at (path , directory , cleanup) }
};
}
