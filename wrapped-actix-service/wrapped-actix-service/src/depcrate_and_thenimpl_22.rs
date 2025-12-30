// Generated macro for impl_22 (impl)
macro_rules! Depcrate_and_thenimpl_22 {
() => {
// Module: crate::and_then
// Provides: {"impl_22"}
// Dependencies: {}
impl < A , B , Req > AndThenServiceFactoryResponse < A , B , Req > where A : ServiceFactory < Req > , B : ServiceFactory < A :: Response > , { fn new (fut_a : A :: Future , fut_b : B :: Future) -> Self { AndThenServiceFactoryResponse { fut_a , fut_b , a : None , b : None , } } }
};
}
