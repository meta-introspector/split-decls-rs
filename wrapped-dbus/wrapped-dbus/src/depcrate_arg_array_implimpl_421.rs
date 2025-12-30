// Generated macro for impl_421 (impl)
macro_rules! Depcrate_arg_array_implimpl_421 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_421"}
// Dependencies: {}
impl < 'a , T : 'a , I : Iterator < Item = T > > Array < 'a , T , I > { # [doc = " Creates a new Array from an iterator."] pub fn new < J : IntoIterator < IntoIter = I , Item = T > > (j : J) -> Array < 'a , T , I > { Array (j . into_iter () , PhantomData) } }
};
}
