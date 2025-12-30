// Generated macro for InitializationType (enum)
macro_rules! Depcrate_slow_vector_initializationInitializationType {
() => {
// Module: crate::slow_vector_initialization
// Provides: {"InitializationType"}
// Dependencies: {}
# [doc = " Type of slow initialization"] enum InitializationType < 'tcx > { # [doc = " Extend is a slow initialization with the form `vec.extend(repeat(0).take(..))`"] Extend (& 'tcx Expr < 'tcx >) , # [doc = " Resize is a slow initialization with the form `vec.resize(.., 0)`"] Resize (& 'tcx Expr < 'tcx >) , }
};
}
