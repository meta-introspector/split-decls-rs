// Generated macro for impl_126 (impl)
macro_rules! Depcrate_lazyimpl_126 {
() => {
// Module: crate::lazy
// Provides: {"impl_126"}
// Dependencies: {}
impl < F , R > Lazy < F , R :: Future > where F : FnOnce () -> R + Send + 'static , R : IntoFuture , { fn get (& mut self) -> & mut R :: Future { match self . inner { _Lazy :: First (_) => { } _Lazy :: Second (ref mut f) => return f , _Lazy :: Moved => panic ! () , } match mem :: replace (& mut self . inner , _Lazy :: Moved) { _Lazy :: First (f) => self . inner = _Lazy :: Second (f () . into_future ()) , _ => panic ! () , } match self . inner { _Lazy :: Second (ref mut f) => f , _ => panic ! () , } } }
};
}
