// Generated macro for impl_423 (impl)
macro_rules! Depcrateimpl_423 {
() => {
// Module: crate
// Provides: {"impl_423"}
// Dependencies: {}
impl < T , E > IntoFuture for Result < T , E > where T : Send + 'static , E : Send + 'static , { type Future = Done < T , E > ; type Item = T ; type Error = E ; fn into_future (self) -> Done < T , E > { done (self) } }
};
}
