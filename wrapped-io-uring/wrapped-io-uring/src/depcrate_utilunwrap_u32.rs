// Generated macro for unwrap_u32 (function)
macro_rules! Depcrate_utilunwrap_u32 {
() => {
// Module: crate::util
// Provides: {"unwrap_u32"}
// Dependencies: {}
# [doc = " Convert a valid `u32` constant."] # [doc = ""] # [doc = " This is a workaround for the lack of panic-in-const in older"] # [doc = " toolchains."] # [allow (unconditional_panic , clippy :: out_of_bounds_indexing)] pub (crate) const fn unwrap_u32 (t : Option < u32 >) -> u32 { match t { Some (v) => v , None => [] [1] , } }
};
}
