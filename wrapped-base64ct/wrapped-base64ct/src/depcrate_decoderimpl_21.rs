// Generated macro for impl_21 (impl)
macro_rules! Depcrate_decoderimpl_21 {
() => {
// Module: crate::decoder
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : Encoding > io :: Read for Decoder < '_ , E > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . is_finished () { return Ok (0) ; } let slice = match buf . get_mut (.. self . remaining_len ()) { Some (bytes) => bytes , None => buf , } ; self . decode (slice) ? ; Ok (slice . len ()) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { if self . is_finished () { return Ok (0) ; } Ok (self . decode_to_end (buf) ? . len ()) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . decode (buf) ? ; Ok (()) } }
};
}
