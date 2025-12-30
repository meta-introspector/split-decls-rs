// Generated macro for impl_13 (impl)
macro_rules! Depcrate_digitalimpl_13 {
() => {
// Module: crate::digital
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Wait + ? Sized > Wait for & mut T { # [inline] async fn wait_for_high (& mut self) -> Result < () , Self :: Error > { T :: wait_for_high (self) . await } # [inline] async fn wait_for_low (& mut self) -> Result < () , Self :: Error > { T :: wait_for_low (self) . await } # [inline] async fn wait_for_rising_edge (& mut self) -> Result < () , Self :: Error > { T :: wait_for_rising_edge (self) . await } # [inline] async fn wait_for_falling_edge (& mut self) -> Result < () , Self :: Error > { T :: wait_for_falling_edge (self) . await } # [inline] async fn wait_for_any_edge (& mut self) -> Result < () , Self :: Error > { T :: wait_for_any_edge (self) . await } }
};
}
