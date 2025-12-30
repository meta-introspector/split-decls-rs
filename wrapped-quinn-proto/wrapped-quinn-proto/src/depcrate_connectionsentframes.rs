// Generated macro for SentFrames (struct)
macro_rules! Depcrate_connectionSentFrames {
() => {
// Module: crate::connection
// Provides: {"SentFrames"}
// Dependencies: {}
# [derive (Default)] struct SentFrames { retransmits : ThinRetransmits , largest_acked : Option < u64 > , stream_frames : StreamMetaVec , # [doc = " Whether the packet contains non-retransmittable frames (like datagrams)"] non_retransmits : bool , requires_padding : bool , }
};
}
