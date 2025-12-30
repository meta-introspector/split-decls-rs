// Generated macro for Outcome (struct)
macro_rules! Depcrate_protocolOutcome {
() => {
// Module: crate::protocol
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of the credentials top-level functions to obtain a complete identity."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Outcome { # [doc = " The identity provide by the helper."] pub identity : gix_sec :: identity :: Account , # [doc = " A handle to the action to perform next in another call to [`helper::invoke()`][crate::helper::invoke()]."] pub next : helper :: NextAction , }
};
}
