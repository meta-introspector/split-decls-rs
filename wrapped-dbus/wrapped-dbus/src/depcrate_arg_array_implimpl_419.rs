// Generated macro for impl_419 (impl)
macro_rules! Depcrate_arg_array_implimpl_419 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_419"}
// Dependencies: {}
impl < 'a , T : Arg + Get < 'a > > Get < 'a > for Vec < T > { fn get (i : & mut Iter < 'a >) -> Option < Self > { < Array < T , Iter < 'a > > > :: get (i) . map (| a | a . collect ()) } }
};
}
