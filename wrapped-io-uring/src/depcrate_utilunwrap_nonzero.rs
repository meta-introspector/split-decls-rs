// Generated macro for unwrap_nonzero (function)
macro_rules! Depcrate_utilunwrap_nonzero {
() => {
// Module: crate::util
// Provides: {"unwrap_nonzero"}
// Dependencies: {}
# [doc = " Convert a valid `NonZeroU32` constant."] # [doc = ""] # [doc = " This is a workaround for the lack of panic-in-const in older"] # [doc = " toolchains."] # [allow (unconditional_panic , clippy :: out_of_bounds_indexing)] pub (crate) const fn unwrap_nonzero (t : Option < NonZeroU32 >) -> NonZeroU32 { match t { Some (v) => v , None => [] [1] , } }
};
}
