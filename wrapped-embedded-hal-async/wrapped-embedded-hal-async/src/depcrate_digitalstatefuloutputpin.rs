// Generated macro for StatefulOutputPin (trait)
macro_rules! Depcrate_digitalStatefulOutputPin {
() => {
// Module: crate::digital
// Provides: {"StatefulOutputPin"}
// Dependencies: {}
# [doc = " Asynchronous push-pull output pin that can read its output state."] pub trait StatefulOutputPin : OutputPin { # [doc = " Is the pin in drive high mode?"] # [doc = ""] # [doc = " This returns [`Ready`](core::task::Poll::Ready) when the pin's drive mode been read."] # [doc = ""] # [doc = " *NOTE* this does *not* read the electrical state of the pin."] async fn is_set_high (& mut self) -> Result < bool , Self :: Error > ; # [doc = " Is the pin in drive low mode?"] # [doc = ""] # [doc = " This returns [`Ready`](core::task::Poll::Ready) when the pin's drive mode been read."] # [doc = ""] # [doc = " *NOTE* this does *not* read the electrical state of the pin."] async fn is_set_low (& mut self) -> Result < bool , Self :: Error > ; # [doc = " Toggle pin output."] # [doc = ""] # [doc = " This returns [`Ready`](core::task::Poll::Ready) when the pin has been toggled."] async fn toggle (& mut self) -> Result < () , Self :: Error > { let was_low : bool = self . is_set_low () . await ? ; self . set_state (PinState :: from (was_low)) . await } }
};
}
