// Generated macro for Segment (struct)
macro_rules! Depcrate_segSegment {
() => {
// Module: crate::seg
// Provides: {"Segment"}
// Dependencies: {}
# [doc = " A CBOR segment"] # [doc = ""] # [doc = " This type represents a single bytes or text segment on the wire. It can be"] # [doc = " read out in parsed chunks based on the size of the input scratch buffer."] pub struct Segment < 'r , R , P > { reader : & 'r mut Decoder < R > , unread : usize , offset : usize , parser : P , }
};
}
