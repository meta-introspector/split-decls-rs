// Generated macro for encoder_function (macro)
macro_rules! Depcrate_macrosencoder_function {
() => {
// Module: crate::macros
// Provides: {"encoder_function"}
// Dependencies: {}
macro_rules ! encoder_function { ($ eof : block , $ body : block , $ slf : ident , $ src_consumed : ident , $ source : ident , $ dest : ident , $ c : ident , $ destination_handle : ident , $ unread_handle : ident , $ destination_check : ident , $ name : ident , $ input : ty , $ source_struct : ident) => (pub fn $ name (& mut $ slf , src : &$ input , dst : & mut [u8] , last : bool) -> (EncoderResult , usize , usize) { let mut $ source = $ source_struct :: new (src) ; let mut $ dest = ByteDestination :: new (dst) ; loop { match $ source . check_available () { Space :: Full ($ src_consumed) => { if last { $ eof } return (EncoderResult :: InputEmpty , $ src_consumed , $ dest . written ()) ; } Space :: Available (source_handle) => { match $ dest .$ destination_check () { Space :: Full (dst_written) => { return (EncoderResult :: OutputFull , source_handle . consumed () , dst_written) ; } Space :: Available ($ destination_handle) => { let ($ c , $ unread_handle) = source_handle . read () ; $ body } } } } } }) ; }
};
}
