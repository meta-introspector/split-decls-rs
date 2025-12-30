// Generated macro for Partial (struct)
macro_rules! Depcrate_codec_framed_readPartial {
() => {
// Module: crate::codec::framed_read
// Provides: {"Partial"}
// Dependencies: {}
# [doc = " Partially loaded headers frame"] # [derive (Debug)] struct Partial { # [doc = " Empty frame"] frame : Continuable , # [doc = " Partial header payload"] buf : BytesMut , continuation_frames_count : usize , }
};
}
