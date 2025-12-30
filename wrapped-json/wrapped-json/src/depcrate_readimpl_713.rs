// Generated macro for impl_713 (impl)
macro_rules! Depcrate_readimpl_713 {
() => {
// Module: crate::read
// Provides: {"impl_713"}
// Dependencies: {}
impl < 'de , R > Read < 'de > for & mut R where R : Read < 'de > , { fn next (& mut self) -> Result < Option < u8 > > { R :: next (self) } fn peek (& mut self) -> Result < Option < u8 > > { R :: peek (self) } fn discard (& mut self) { R :: discard (self) ; } fn position (& self) -> Position { R :: position (self) } fn peek_position (& self) -> Position { R :: peek_position (self) } fn byte_offset (& self) -> usize { R :: byte_offset (self) } fn parse_str < 's > (& 's mut self , scratch : & 's mut Vec < u8 >) -> Result < Reference < 'de , 's , str > > { R :: parse_str (self , scratch) } fn parse_str_raw < 's > (& 's mut self , scratch : & 's mut Vec < u8 > ,) -> Result < Reference < 'de , 's , [u8] > > { R :: parse_str_raw (self , scratch) } fn ignore_str (& mut self) -> Result < () > { R :: ignore_str (self) } fn decode_hex_escape (& mut self) -> Result < u16 > { R :: decode_hex_escape (self) } # [cfg (feature = "raw_value")] fn begin_raw_buffering (& mut self) { R :: begin_raw_buffering (self) ; } # [cfg (feature = "raw_value")] fn end_raw_buffering < V > (& mut self , visitor : V) -> Result < V :: Value > where V : Visitor < 'de > , { R :: end_raw_buffering (self , visitor) } const should_early_return_if_failed : bool = R :: should_early_return_if_failed ; fn set_failed (& mut self , failed : & mut bool) { R :: set_failed (self , failed) ; } }
};
}
