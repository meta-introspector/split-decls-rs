// Generated macro for impl_186 (impl)
macro_rules! Depcrate_hpack_huffmanimpl_186 {
() => {
// Module: crate::hpack::huffman
// Provides: {"impl_186"}
// Dependencies: {}
impl Decoder { fn new () -> Decoder { Decoder { state : 0 , maybe_eos : false , } } fn decode4 (& mut self , input : u8) -> Result < Option < u8 > , DecoderError > { let (next , byte , flags) = DECODE_TABLE [self . state] [input as usize] ; if flags & ERROR == ERROR { return Err (DecoderError :: InvalidHuffmanCode) ; } let mut ret = None ; if flags & DECODED == DECODED { ret = Some (byte) ; } self . state = next ; self . maybe_eos = flags & MAYBE_EOS == MAYBE_EOS ; Ok (ret) } fn is_final (& self) -> bool { self . state == 0 || self . maybe_eos } }
};
}
