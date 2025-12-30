// Generated macro for impl_424 (impl)
macro_rules! Depcrate_arg_array_implimpl_424 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_424"}
// Dependencies: {}
impl < 'a , T : Arg + Get < 'a > > Get < 'a > for Array < 'a , T , Iter < 'a > > { fn get (i : & mut Iter < 'a >) -> Option < Array < 'a , T , Iter < 'a > > > { i . recurse (Self :: ARG_TYPE) . map (| si | Array (si , PhantomData)) } }
};
}
