// Generated macro for impl_760 (impl)
macro_rules! Depcrate_generatedimpl_760 {
() => {
// Module: crate::generated
// Provides: {"impl_760"}
// Dependencies: {}
impl NWUDPSessionState { # [doc (alias = "NWUDPSessionStateInvalid")] # [deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"] pub const Invalid : Self = Self (0) ; # [doc = " attempting to make the session ready."] # [doc (alias = "NWUDPSessionStateWaiting")] # [deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"] pub const Waiting : Self = Self (1) ; # [doc (alias = "NWUDPSessionStatePreparing")] # [deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"] pub const Preparing : Self = Self (2) ; # [doc (alias = "NWUDPSessionStateReady")] # [deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"] pub const Ready : Self = Self (3) ; # [doc = " at this time, either due to problems with the path or the client rejecting the"] # [doc = " endpoints."] # [doc (alias = "NWUDPSessionStateFailed")] # [deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"] pub const Failed : Self = Self (4) ; # [doc (alias = "NWUDPSessionStateCancelled")] # [deprecated = "Use `nw_connection_state_t` in Network framework instead, see deprecation notice in <NetworkExtension/NWUDPSession.h>"] pub const Cancelled : Self = Self (5) ; }
};
}
