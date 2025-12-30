// Generated macro for impl_11 (impl)
macro_rules! Depcrate_digitalimpl_11 {
() => {
// Module: crate::digital
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : InputPin + ? Sized > InputPin for & mut T { # [inline] async fn is_high (& mut self) -> Result < bool , Self :: Error > { T :: is_high (self) . await } # [inline] async fn is_low (& mut self) -> Result < bool , Self :: Error > { T :: is_low (self) . await } }
};
}
