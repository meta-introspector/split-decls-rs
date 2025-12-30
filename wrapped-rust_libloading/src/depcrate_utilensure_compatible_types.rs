// Generated macro for ensure_compatible_types (function)
macro_rules! Depcrate_utilensure_compatible_types {
() => {
// Module: crate::util
// Provides: {"ensure_compatible_types"}
// Dependencies: {}
# [inline] pub (crate) fn ensure_compatible_types < T , E > () -> Result < () , Error > { if size_of :: < T > () != size_of :: < E > () { Err (Error :: IncompatibleSize) } else { Ok (()) } }
};
}
