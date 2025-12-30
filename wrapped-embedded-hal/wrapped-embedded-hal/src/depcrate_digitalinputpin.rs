// Generated macro for InputPin (trait)
macro_rules! Depcrate_digitalInputPin {
() => {
// Module: crate::digital
// Provides: {"InputPin"}
// Dependencies: {}
# [doc = " Single digital input pin."] pub trait InputPin : ErrorType { # [doc = " Is the input pin high?"] fn is_high (& mut self) -> Result < bool , Self :: Error > ; # [doc = " Is the input pin low?"] fn is_low (& mut self) -> Result < bool , Self :: Error > ; }
};
}
