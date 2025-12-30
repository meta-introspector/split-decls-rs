// Generated macro for impl_504 (impl)
macro_rules! Depcrate_routineimpl_504 {
() => {
// Module: crate::routine
// Provides: {"impl_504"}
// Dependencies: {}
impl < M : Measurement , F , T > Function < M , F , T > where F : FnMut (& mut Bencher < '_ , M > , & T) , T : ? Sized , { pub fn new (f : F) -> Function < M , F , T > { Function { f , _phantom : PhantomData , _phamtom2 : PhantomData , } } }
};
}
