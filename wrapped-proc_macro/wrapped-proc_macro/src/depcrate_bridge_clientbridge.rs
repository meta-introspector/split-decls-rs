// Generated macro for Bridge (struct)
macro_rules! Depcrate_bridge_clientBridge {
() => {
// Module: crate::bridge::client
// Provides: {"Bridge"}
// Dependencies: {}
struct Bridge < 'a > { # [doc = " Reusable buffer (only `clear`-ed, never shrunk), primarily"] # [doc = " used for making requests."] cached_buffer : Buffer , # [doc = " Server-side function that the client uses to make requests."] dispatch : closure :: Closure < 'a , Buffer , Buffer > , # [doc = " Provided globals for this macro expansion."] globals : ExpnGlobals < Span > , }
};
}
