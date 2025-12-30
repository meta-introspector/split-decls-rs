// Generated macro for decoder_function (macro)
macro_rules! Depcrate_macrosdecoder_function {
() => {
// Module: crate::macros
// Provides: {"decoder_function"}
// Dependencies: {}
macro_rules ! decoder_function { ($ preamble : block , $ loop_preamble : block , $ eof : block , $ body : block , $ slf : ident , $ src_consumed : ident , $ dest : ident , $ source : ident , $ b : ident , $ destination_handle : ident , $ unread_handle : ident , $ destination_check : ident , $ name : ident , $ code_unit : ty , $ dest_struct : ident) => (pub fn $ name (& mut $ slf , src : & [u8] , dst : & mut [$ code_unit] , last : bool) -> (DecoderResult , usize , usize) { let mut $ source = ByteSource :: new (src) ; let mut $ dest = $ dest_struct :: new (dst) ; loop { { $ preamble } loop { { $ loop_preamble } match $ source . check_available () { Space :: Full ($ src_consumed) => { if last { $ eof } return (DecoderResult :: InputEmpty , $ src_consumed , $ dest . written ()) ; } Space :: Available (source_handle) => { match $ dest .$ destination_check () { Space :: Full (dst_written) => { return (DecoderResult :: OutputFull , source_handle . consumed () , dst_written) ; } Space :: Available ($ destination_handle) => { let ($ b , $ unread_handle) = source_handle . read () ; $ body } } } } } } }) ; }
};
}
