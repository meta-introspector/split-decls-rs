// Generated macro for impl_9 (impl)
macro_rules! Depcrate_digitalimpl_9 {
() => {
// Module: crate::digital
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : StatefulOutputPin + ? Sized > StatefulOutputPin for & mut T { # [inline] async fn is_set_high (& mut self) -> Result < bool , Self :: Error > { T :: is_set_high (self) . await } # [inline] async fn is_set_low (& mut self) -> Result < bool , Self :: Error > { T :: is_set_low (self) . await } # [inline] async fn toggle (& mut self) -> Result < () , Self :: Error > { T :: toggle (self) . await } }
};
}
