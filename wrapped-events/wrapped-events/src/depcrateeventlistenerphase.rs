// Generated macro for EventListenerPhase (enum)
macro_rules! DepcrateEventListenerPhase {
() => {
// Module: crate
// Provides: {"EventListenerPhase"}
// Dependencies: {}
# [doc = " Specifies whether the event listener is run during the capture or bubble phase."] # [doc = ""] # [doc = " The official specification has [a good explanation](https://www.w3.org/TR/DOM-Level-3-Events/#event-flow)"] # [doc = " of capturing vs bubbling."] # [doc = ""] # [doc = " # Default"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use gloo_events::EventListenerPhase;"] # [doc = " #"] # [doc = " EventListenerPhase::Bubble"] # [doc = " # ;"] # [doc = " ```"] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] pub enum EventListenerPhase { # [default] # [allow (missing_docs)] Bubble , # [allow (missing_docs)] Capture , }
};
}
