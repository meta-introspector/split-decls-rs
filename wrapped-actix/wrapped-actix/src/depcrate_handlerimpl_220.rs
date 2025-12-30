// Generated macro for impl_220 (impl)
macro_rules! Depcrate_handlerimpl_220 {
() => {
// Module: crate::handler
// Provides: {"impl_220"}
// Dependencies: {}
impl < A : Actor , I > ActorResponse < A , I > { # [doc = " Creates a response."] pub fn reply (val : I) -> Self { Self { item : ActorResponseTypeItem :: Result (val) , } } # [doc = " Creates an asynchronous response."] pub fn r#async < T > (fut : T) -> Self where T : ActorFuture < A , Output = I > + 'static , { Self { item : ActorResponseTypeItem :: Fut (Box :: pin (fut)) , } } }
};
}
