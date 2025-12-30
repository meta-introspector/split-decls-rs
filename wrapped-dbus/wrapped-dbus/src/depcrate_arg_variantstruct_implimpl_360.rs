// Generated macro for impl_360 (impl)
macro_rules! Depcrate_arg_variantstruct_implimpl_360 {
() => {
// Module: crate::arg::variantstruct_impl
// Provides: {"impl_360"}
// Dependencies: {}
impl Variant < Box < dyn RefArg > > { # [doc = " Creates a new refarg from an Iter. Mainly for internal use."] pub fn new_refarg < 'a > (i : & mut Iter < 'a >) -> Option < Self > { i . recurse (ArgType :: Variant) . and_then (| mut si | si . get_refarg ()) . map (Variant) } }
};
}
