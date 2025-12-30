// Generated macro for impl_121 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_121 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_121"}
// Dependencies: {}
impl < R > CombineBuffer < BufReader < R > > for Bufferless { fn buffer < 'a > (& 'a self , read : & 'a BufReader < R >) -> & 'a [u8] { & read . buf } fn advance (& mut self , read : & mut BufReader < R > , len : usize) { read . buf . advance (len) ; } # [cfg (feature = "pin-project-lite")] fn advance_pin (& mut self , read : Pin < & mut BufReader < R > > , len : usize) { read . project () . buf . advance (len) ; } }
};
}
