// Generated macro for impl_7 (impl)
macro_rules! Depcrate_digitalimpl_7 {
() => {
// Module: crate::digital
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : OutputPin + ? Sized > OutputPin for & mut T { # [inline] async fn set_low (& mut self) -> Result < () , Self :: Error > { T :: set_low (self) . await } # [inline] async fn set_high (& mut self) -> Result < () , Self :: Error > { T :: set_high (self) . await } # [inline] async fn set_state (& mut self , state : PinState) -> Result < () , Self :: Error > { T :: set_state (self , state) . await } }
};
}
