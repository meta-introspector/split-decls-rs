// Generated macro for impl_215 (impl)
macro_rules! Depcrate_handlerimpl_215 {
() => {
// Module: crate::handler
// Provides: {"impl_215"}
// Dependencies: {}
impl < I > Response < I > { # [doc = " Creates an asynchronous response."] pub fn fut < T > (fut : T) -> Self where T : Future < Output = I > + 'static , { Self { item : ResponseTypeItem :: Fut (Box :: pin (fut)) , } } # [doc = " Creates a response."] pub fn reply (val : I) -> Self { Self { item : ResponseTypeItem :: Result (val) , } } }
};
}
