// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl EventListenerOptions { # [doc = " Returns an `EventListenerOptions` with `phase` set to `EventListenerPhase::Capture`."] # [doc = ""] # [doc = " This is the same as:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use gloo_events::{EventListenerOptions, EventListenerPhase};"] # [doc = " #"] # [doc = " EventListenerOptions {"] # [doc = "     phase: EventListenerPhase::Capture,"] # [doc = "     ..Default::default()"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] # [inline] pub fn run_in_capture_phase () -> Self { Self { phase : EventListenerPhase :: Capture , .. Self :: default () } } # [doc = " Returns an `EventListenerOptions` with `passive` set to `false`."] # [doc = ""] # [doc = " This is the same as:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use gloo_events::EventListenerOptions;"] # [doc = " #"] # [doc = " EventListenerOptions {"] # [doc = "     passive: false,"] # [doc = "     ..Default::default()"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] # [inline] pub fn enable_prevent_default () -> Self { Self { passive : false , .. Self :: default () } } # [inline] fn as_js (& self , once : bool) -> AddEventListenerOptions { let mut options = AddEventListenerOptions :: new () ; options . capture (self . phase . is_capture ()) ; options . once (once) ; options . passive (self . passive) ; options } }
};
}
