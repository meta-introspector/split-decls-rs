// Generated macro for OutputPin (trait)
macro_rules! Depcrate_digitalOutputPin {
() => {
// Module: crate::digital
// Provides: {"OutputPin"}
// Dependencies: {}
# [doc = " Single digital push-pull output pin."] pub trait OutputPin : ErrorType { # [doc = " Drives the pin low."] # [doc = ""] # [doc = " *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external"] # [doc = " electrical sources."] fn set_low (& mut self) -> Result < () , Self :: Error > ; # [doc = " Drives the pin high."] # [doc = ""] # [doc = " *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external"] # [doc = " electrical sources."] fn set_high (& mut self) -> Result < () , Self :: Error > ; # [doc = " Drives the pin high or low depending on the provided value."] # [doc = ""] # [doc = " *NOTE* the actual electrical state of the pin may not actually be high or low, e.g. due to external"] # [doc = " electrical sources."] # [inline] fn set_state (& mut self , state : PinState) -> Result < () , Self :: Error > { match state { PinState :: Low => self . set_low () , PinState :: High => self . set_high () , } } }
};
}
