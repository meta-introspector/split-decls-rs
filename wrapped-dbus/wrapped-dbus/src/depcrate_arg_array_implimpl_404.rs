// Generated macro for impl_404 (impl)
macro_rules! Depcrate_arg_array_implimpl_404 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_404"}
// Dependencies: {}
impl < 'a , K : 'a + DictKey , V : 'a + Append + Arg , I : Iterator < Item = (K , V) > > Dict < 'a , K , V , I > { # [doc = " Creates a new Dict from an iterator."] pub fn new < J : IntoIterator < IntoIter = I , Item = (K , V) > > (j : J) -> Dict < 'a , K , V , I > { Dict (j . into_iter () , PhantomData) } }
};
}
