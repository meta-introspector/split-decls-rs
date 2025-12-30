// Generated macro for InputPin (trait)
macro_rules! Depcrate_digitalInputPin {
() => {
// Module: crate::digital
// Provides: {"InputPin"}
// Dependencies: {}
# [doc = " Asynchronous single digital input pin."] pub trait InputPin : ErrorType { # [doc = " Is the input pin high?"] # [doc = ""] # [doc = " This returns [`Ready`](core::task::Poll::Ready) when the pin's electrical state has been read."] # [doc = ""] # [doc = " *NOTE* the input state of the pin may have changed before the future is polled."] async fn is_high (& mut self) -> Result < bool , Self :: Error > ; # [doc = " Is the input pin low?"] # [doc = ""] # [doc = " This returns [`Ready`](core::task::Poll::Ready) when the pin's electrical state has been read."] # [doc = ""] # [doc = " *NOTE* the input state of the pin may have changed before the future is polled."] async fn is_low (& mut self) -> Result < bool , Self :: Error > ; }
};
}
