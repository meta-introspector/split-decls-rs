// Generated macro for impl_62 (impl)
macro_rules! Depcrate_eventsimpl_62 {
() => {
// Module: crate::events
// Provides: {"impl_62"}
// Dependencies: {}
impl EventData { # [doc = " Returns size of `EventData` array of `QuicFrame`s if it exists."] pub fn contains_quic_frames (& self) -> Option < usize > { match self { EventData :: PacketSent (pkt) => pkt . frames . as_ref () . map (| f | f . len ()) , EventData :: PacketReceived (pkt) => pkt . frames . as_ref () . map (| f | f . len ()) , EventData :: PacketLost (pkt) => pkt . frames . as_ref () . map (| f | f . len ()) , EventData :: MarkedForRetransmit (ev) => Some (ev . frames . len ()) , EventData :: FramesProcessed (ev) => Some (ev . frames . len ()) , _ => None , } } }
};
}
