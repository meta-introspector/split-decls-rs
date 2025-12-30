// Generated macro for impl_134 (impl)
macro_rules! Depcrate_mapimpl_134 {
() => {
// Module: crate::map
// Provides: {"impl_134"}
// Dependencies: {}
impl < A , F , Req , Res > MapServiceFuture < A , F , Req , Res > where A : ServiceFactory < Req > , F : FnMut (A :: Response) -> Res , { fn new (fut : A :: Future , f : F) -> Self { MapServiceFuture { f : Some (f) , fut } } }
};
}
