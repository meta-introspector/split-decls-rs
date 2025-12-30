// Generated macro for impl_51 (impl)
macro_rules! Depcrate_dequeimpl_51 {
() => {
// Module: crate::deque
// Provides: {"impl_51"}
// Dependencies: {}
impl < T > Default for Injector < T > { fn default () -> Self { let block = Box :: into_raw (Block :: < T > :: new ()) ; Self { head : CachePadded :: new (Position { block : AtomicPtr :: new (block) , index : AtomicUsize :: new (0) , }) , tail : CachePadded :: new (Position { block : AtomicPtr :: new (block) , index : AtomicUsize :: new (0) , }) , _marker : PhantomData , } } }
};
}
