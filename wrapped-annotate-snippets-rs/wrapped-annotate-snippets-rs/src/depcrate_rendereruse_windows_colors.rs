// Generated macro for USE_WINDOWS_COLORS (const)
macro_rules! Depcrate_rendererUSE_WINDOWS_COLORS {
() => {
// Module: crate::renderer
// Provides: {"USE_WINDOWS_COLORS"}
// Dependencies: {}
const USE_WINDOWS_COLORS : bool = cfg ! (windows) && ! cfg ! (feature = "testing-colors") ;
};
}
