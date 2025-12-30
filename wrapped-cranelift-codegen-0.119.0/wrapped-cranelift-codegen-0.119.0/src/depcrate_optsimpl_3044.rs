// Generated macro for impl_3044 (impl)
macro_rules! Depcrate_optsimpl_3044 {
() => {
// Module: crate::opts
// Provides: {"impl_3044"}
// Dependencies: {}
impl < 'a , 'b , 'c > InstDataEtorIter < 'a , 'b , 'c > { fn new (root : Value) -> Self { debug_assert_ne ! (root , Value :: reserved_value ()) ; trace ! ("new iter from root {root}") ; Self { stack : smallvec ! [root] , _phantom1 : PhantomData , _phantom2 : PhantomData , _phantom3 : PhantomData , } } }
};
}
