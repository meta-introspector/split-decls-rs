// Generated macro for impl_234 (impl)
macro_rules! Depcrate_transformimpl_234 {
() => {
// Module: crate::transform
// Provides: {"impl_234"}
// Dependencies: {}
impl < T , S , Req > ApplyTransform < T , S , Req > where S : ServiceFactory < Req > , T : Transform < S :: Service , Req , InitError = S :: InitError > , { # [doc = " Create new `ApplyTransform` new service instance"] fn new (t : T , service : S) -> Self { Self (Rc :: new ((t , service)) , PhantomData) } }
};
}
