// Generated macro for Data (struct)
macro_rules! Depcrate_frame_dataData {
() => {
// Module: crate::frame::data
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Data frame"] # [doc = ""] # [doc = " Data frames convey arbitrary, variable-length sequences of octets associated"] # [doc = " with a stream. One or more DATA frames are used, for instance, to carry HTTP"] # [doc = " request or response payloads."] # [derive (Eq , PartialEq)] pub struct Data < T = Bytes > { stream_id : StreamId , data : T , flags : DataFlags , pad_len : Option < u8 > , }
};
}
