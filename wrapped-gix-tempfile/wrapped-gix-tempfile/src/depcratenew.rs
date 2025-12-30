// Generated macro for new (function)
macro_rules! Depcratenew {
() => {
// Module: crate
// Provides: {"new"}
// Dependencies: {}
# [doc = " A shortcut to [`Handle::<Writable>::new()`], creating a writable temporary file with non-clashing name in a directory."] pub fn new (containing_directory : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove ,) -> io :: Result < Handle < Writable > > { Handle :: < Writable > :: new (containing_directory , directory , cleanup) }
};
}
