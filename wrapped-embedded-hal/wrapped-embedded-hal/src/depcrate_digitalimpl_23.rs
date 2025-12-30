// Generated macro for impl_23 (impl)
macro_rules! Depcrate_digitalimpl_23 {
() => {
// Module: crate::digital
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : OutputPin + ? Sized > OutputPin for & mut T { # [inline] fn set_low (& mut self) -> Result < () , Self :: Error > { T :: set_low (self) } # [inline] fn set_high (& mut self) -> Result < () , Self :: Error > { T :: set_high (self) } # [inline] fn set_state (& mut self , state : PinState) -> Result < () , Self :: Error > { T :: set_state (self , state) } }
};
}
