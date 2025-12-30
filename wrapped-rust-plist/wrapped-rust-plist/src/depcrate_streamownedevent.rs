// Generated macro for OwnedEvent (type)
macro_rules! Depcrate_streamOwnedEvent {
() => {
// Module: crate::stream
// Provides: {"OwnedEvent"}
// Dependencies: {}
# [doc = " An owned [`Event`]."] # [doc = ""] # [doc = " During deserialization, events are always owned; this type alias helps"] # [doc = " keep that code a bit clearer."] pub type OwnedEvent = Event < 'static > ;
};
}
