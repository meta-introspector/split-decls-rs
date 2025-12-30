// Generated macro for impl_488 (impl)
macro_rules! Depcrate_reader_stateimpl_488 {
() => {
// Module: crate::reader::state
// Provides: {"impl_488"}
// Dependencies: {}
impl Default for ReaderState { fn default () -> Self { Self { offset : 0 , last_error_offset : 0 , state : ParseState :: Init , config : Config :: default () , opened_buffer : Vec :: new () , opened_starts : Vec :: new () , # [cfg (feature = "encoding")] encoding : EncodingRef :: Implicit (UTF_8) , } } }
};
}
