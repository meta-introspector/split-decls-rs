// Generated macro for impl_159 (impl)
macro_rules! Depcrate_stream_rzcobsimpl_159 {
() => {
// Module: crate::stream::rzcobs
// Provides: {"impl_159"}
// Dependencies: {}
impl StreamDecoder for Rzcobs < '_ > { fn received (& mut self , mut data : & [u8]) { if self . raw . is_empty () { while data . first () == Some (& 0) { data = & data [1 ..] } } self . raw . extend_from_slice (data) ; } fn decode (& mut self) -> Result < Frame < '_ > , DecodeError > { let zero = self . raw . iter () . position (| & x | x == 0) . ok_or (DecodeError :: UnexpectedEof) ? ; let frame = rzcobs_decode (& self . raw [.. zero]) ; if let Some (nonzero) = self . raw [zero ..] . iter () . position (| & x | x != 0) { self . raw . drain (0 .. zero + nonzero) ; } else { self . raw . clear () ; } assert ! (self . raw . is_empty () || self . raw [0] != 0) ; let frame : Vec < u8 > = frame ? ; match self . table . decode (& frame) { Ok ((frame , _consumed)) => Ok (frame) , Err (DecodeError :: UnexpectedEof) => Err (DecodeError :: Malformed) , Err (DecodeError :: Malformed) => Err (DecodeError :: Malformed) , } } }
};
}
