// Generated macro for map_ok_or_else_fn (function)
macro_rules! Depcrate_fnsmap_ok_or_else_fn {
() => {
// Module: crate::fns
// Provides: {"map_ok_or_else_fn"}
// Dependencies: {}
pub (crate) fn map_ok_or_else_fn < F , G > (f : F , g : G) -> MapOkOrElseFn < F , G > { chain_fn (map_ok_fn (f) , chain_fn (map_err_fn (g) , merge_result_fn ())) }
};
}
