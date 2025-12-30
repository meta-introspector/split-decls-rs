// Generated macro for NextResolve (struct)
macro_rules! Depcrate_extensionsNextResolve {
() => {
// Module: crate::extensions
// Provides: {"NextResolve"}
// Dependencies: {}
# [doc = " The remainder of a extension chain for resolve."] pub struct NextResolve < 'a > { chain : & 'a [Arc < dyn Extension >] , resolve_fut : ResolveFut < 'a > , }
};
}
