// Generated macro for mark_at (function)
macro_rules! Depcratemark_at {
() => {
// Module: crate
// Provides: {"mark_at"}
// Dependencies: {}
# [doc = " A shortcut to [`Handle::<Closed>::at()`] providing a closed temporary file to mark the presence of something."] pub fn mark_at (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove ,) -> io :: Result < Handle < Closed > > { Handle :: < Closed > :: at (path , directory , cleanup) }
};
}
