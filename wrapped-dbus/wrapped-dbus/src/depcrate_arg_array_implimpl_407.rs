// Generated macro for impl_407 (impl)
macro_rules! Depcrate_arg_array_implimpl_407 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_407"}
// Dependencies: {}
impl < 'a , K : DictKey + Get < 'a > , V : Arg + Get < 'a > > Get < 'a > for Dict < 'a , K , V , Iter < 'a > > { fn get (i : & mut Iter < 'a >) -> Option < Self > { i . recurse (Self :: ARG_TYPE) . map (| si | Dict (si , PhantomData)) } }
};
}
