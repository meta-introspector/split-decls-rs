// Generated macro for impl_27 (impl)
macro_rules! Depcrate_digitalimpl_27 {
() => {
// Module: crate::digital
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : InputPin + ? Sized > InputPin for & mut T { # [inline] fn is_high (& mut self) -> Result < bool , Self :: Error > { T :: is_high (self) } # [inline] fn is_low (& mut self) -> Result < bool , Self :: Error > { T :: is_low (self) } }
};
}
