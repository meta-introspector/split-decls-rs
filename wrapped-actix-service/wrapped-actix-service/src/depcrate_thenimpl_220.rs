// Generated macro for impl_220 (impl)
macro_rules! Depcrate_thenimpl_220 {
() => {
// Module: crate::then
// Provides: {"impl_220"}
// Dependencies: {}
impl < A , B , Req > ThenServiceFactoryResponse < A , B , Req > where A : ServiceFactory < Req > , B : ServiceFactory < Result < A :: Response , A :: Error > , Config = A :: Config , Error = A :: Error , InitError = A :: InitError , > , { fn new (fut_a : A :: Future , fut_b : B :: Future) -> Self { Self { fut_a , fut_b , a : None , b : None , } } }
};
}
