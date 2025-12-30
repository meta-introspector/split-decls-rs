// Generated macro for Segments (struct)
macro_rules! Depcrate_segSegments {
() => {
// Module: crate::seg
// Provides: {"Segments"}
// Dependencies: {}
# [doc = " A sequence of CBOR segments"] # [doc = ""] # [doc = " CBOR allows for bytes or text items to be segmented. This type represents"] # [doc = " the state of that segmented input stream."] pub struct Segments < 'r , R , P > { reader : & 'r mut Decoder < R > , state : State , parser : PhantomData < P > , unwrap : fn (Header) -> Result < Option < usize > , () > , }
};
}
