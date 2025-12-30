// Generated macro for impl_152 (impl)
macro_rules! Depcrate_stream_rawimpl_152 {
() => {
// Module: crate::stream::raw
// Provides: {"impl_152"}
// Dependencies: {}
impl StreamDecoder for Raw < '_ > { fn received (& mut self , data : & [u8]) { self . data . extend_from_slice (data) ; } fn decode (& mut self) -> Result < Frame < '_ > , DecodeError > { match self . table . decode (& self . data) { Ok ((frame , consumed)) => { self . data . drain (0 .. consumed) ; Ok (frame) } Err (e) => Err (e) , } } }
};
}
