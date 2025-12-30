// Generated macro for DecoderError (enum)
macro_rules! Depcrate_hpack_decoderDecoderError {
() => {
// Module: crate::hpack::decoder
// Provides: {"DecoderError"}
// Dependencies: {}
# [doc = " Represents all errors that can be encountered while performing the decoding"] # [doc = " of an HPACK header set."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum DecoderError { InvalidRepresentation , InvalidIntegerPrefix , InvalidTableIndex , InvalidHuffmanCode , InvalidUtf8 , InvalidStatusCode , InvalidPseudoheader , InvalidMaxDynamicSize , IntegerOverflow , NeedMore (NeedMore) , }
};
}
