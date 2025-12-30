// Generated macro for Event (enum)
macro_rules! DepcrateEvent {
() => {
// Module: crate
// Provides: {"Event"}
// Dependencies: {}
# [doc = " The core netlog event type with several domain-specific variants."] # [derive (Debug)] pub enum Event { Http (http :: Event) , H2 (h2 :: Event) , H3 (h3 :: Event) , Quic (quic :: Event) , }
};
}
