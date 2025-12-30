// Generated macro for impl_127 (impl)
macro_rules! Depcrate_mapimpl_127 {
() => {
// Module: crate::map
// Provides: {"impl_127"}
// Dependencies: {}
impl < A , F , Req , Res > MapFuture < A , F , Req , Res > where A : Service < Req > , F : FnMut (A :: Response) -> Res , { fn new (fut : A :: Future , f : F) -> Self { MapFuture { f , fut } } }
};
}
