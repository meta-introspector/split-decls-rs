// Generated macro for GzState (struct)
macro_rules! Depcrate_gzGzState {
() => {
// Module: crate::gz
// Provides: {"GzState"}
// Dependencies: {}
# [repr (C)] struct GzState { have : c_uint , next : * const Bytef , pos : i64 , mode : GzMode , fd : c_int , source : Source , want : usize , input : * mut u8 , in_size : usize , output : * mut u8 , out_size : usize , direct : bool , how : How , start : i64 , eof : bool , past : bool , level : i8 , strategy : Strategy , reset : bool , skip : i64 , seek : bool , err : c_int , msg : * const c_char , stream : z_stream , }
};
}
