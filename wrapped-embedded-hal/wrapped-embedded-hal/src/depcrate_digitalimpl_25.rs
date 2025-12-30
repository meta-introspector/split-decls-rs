// Generated macro for impl_25 (impl)
macro_rules! Depcrate_digitalimpl_25 {
() => {
// Module: crate::digital
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : StatefulOutputPin + ? Sized > StatefulOutputPin for & mut T { # [inline] fn is_set_high (& mut self) -> Result < bool , Self :: Error > { T :: is_set_high (self) } # [inline] fn is_set_low (& mut self) -> Result < bool , Self :: Error > { T :: is_set_low (self) } # [inline] fn toggle (& mut self) -> Result < () , Self :: Error > { T :: toggle (self) } }
};
}
