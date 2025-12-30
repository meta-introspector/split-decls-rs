// Generated macro for SendBlink (struct)
macro_rules! Depcrate_blinkSendBlink {
() => {
// Module: crate::blink
// Provides: {"SendBlink"}
// Dependencies: {}
# [doc = " Wrapper for [`Blink`] that implements [`Send`]."] # [doc = ""] # [doc = " Normally it is impossible to send [`Blink`] to another thread"] # [doc = " due to the fact that it will drop non-sendable types on reset."] # [doc = ""] # [doc = " This wrapper resets [`Blink`] on construction and thus safe to send."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"alloc\")] fn main() {"] # [doc = " # use blink_alloc::{SendBlink, Blink};"] # [doc = " let mut blink = Blink::new();"] # [doc = " let rc = std::rc::Rc::new(42);"] # [doc = " let rc = blink.put(rc);"] # [doc = " assert_eq!(**rc, 42);"] # [doc = " let send_blink = SendBlink::new(blink);"] # [doc = ""] # [doc = " std::thread::scope(move |_| {"] # [doc = "     let mut blink = send_blink.into_inner();"] # [doc = "     blink.put(42);"] # [doc = " });"] # [doc = " # }"] # [doc = " # #[cfg(not(feature = \"alloc\"))] fn main() {}"] # [doc = " ````"] pub struct SendBlink < A > { blink : Blink < A > , }
};
}
