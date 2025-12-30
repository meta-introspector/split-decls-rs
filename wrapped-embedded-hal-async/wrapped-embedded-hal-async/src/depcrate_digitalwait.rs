// Generated macro for Wait (trait)
macro_rules! Depcrate_digitalWait {
() => {
// Module: crate::digital
// Provides: {"Wait"}
// Dependencies: {}
# [doc = " Asynchronously wait for GPIO pin state."] pub trait Wait : ErrorType { # [doc = " Wait until the pin is high. If it is already high, return immediately."] # [doc = ""] # [doc = " # Note for implementers"] # [doc = " The pin may have switched back to low before the task was run after"] # [doc = " being woken. The future should still resolve in that case."] async fn wait_for_high (& mut self) -> Result < () , Self :: Error > ; # [doc = " Wait until the pin is low. If it is already low, return immediately."] # [doc = ""] # [doc = " # Note for implementers"] # [doc = " The pin may have switched back to high before the task was run after"] # [doc = " being woken. The future should still resolve in that case."] async fn wait_for_low (& mut self) -> Result < () , Self :: Error > ; # [doc = " Wait for the pin to undergo a transition from low to high."] # [doc = ""] # [doc = " If the pin is already high, this does *not* return immediately, it'll wait for the"] # [doc = " pin to go low and then high again."] async fn wait_for_rising_edge (& mut self) -> Result < () , Self :: Error > ; # [doc = " Wait for the pin to undergo a transition from high to low."] # [doc = ""] # [doc = " If the pin is already low, this does *not* return immediately, it'll wait for the"] # [doc = " pin to go high and then low again."] async fn wait_for_falling_edge (& mut self) -> Result < () , Self :: Error > ; # [doc = " Wait for the pin to undergo any transition, i.e low to high OR high to low."] async fn wait_for_any_edge (& mut self) -> Result < () , Self :: Error > ; }
};
}
