// Generated macro for impl_365 (impl)
macro_rules! Depcrate_arg_variantstruct_implimpl_365 {
() => {
// Module: crate::arg::variantstruct_impl
// Provides: {"impl_365"}
// Dependencies: {}
impl < 'a , T : Get < 'a > > Get < 'a > for Variant < T > { fn get (i : & mut Iter < 'a >) -> Option < Variant < T > > { i . recurse (ArgType :: Variant) . and_then (| mut si | si . get () . map (Variant)) } }
};
}
