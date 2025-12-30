// Generated macro for InitializedSize (enum)
macro_rules! Depcrate_slow_vector_initializationInitializedSize {
() => {
// Module: crate::slow_vector_initialization
// Provides: {"InitializedSize"}
// Dependencies: {}
# [doc = " Initializer for the creation of the vector."] # [doc = ""] # [doc = " When `Vec::with_capacity(size)` is found, the `size` expression will be in"] # [doc = " `InitializedSize::Initialized`."] # [doc = ""] # [doc = " Otherwise, for `Vec::new()` calls, there is no allocation initializer yet, so"] # [doc = " `InitializedSize::Uninitialized` is used."] # [doc = " Later, when a call to `.resize(size, 0)` or similar is found, it's set"] # [doc = " to `InitializedSize::Initialized(size)`."] # [doc = ""] # [doc = " Since it will be set to `InitializedSize::Initialized(size)` when a slow initialization is"] # [doc = " found, it is always safe to \"unwrap\" it at lint time."] enum InitializedSize < 'tcx > { Initialized (& 'tcx Expr < 'tcx >) , Uninitialized , }
};
}
