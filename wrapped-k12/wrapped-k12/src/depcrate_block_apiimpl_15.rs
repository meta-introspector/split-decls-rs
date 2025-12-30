// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl ExtendableOutputCore for KangarooTwelveCore < '_ > { type ReaderCore = KangarooTwelveReaderCore ; # [inline] fn finalize_xof_core (& mut self , buffer : & mut Buffer < Self >) -> Self :: ReaderCore { let mut lenbuf = [0u8 ; LENGTH_ENCODE_SIZE] ; buffer . digest_blocks (self . customization , | block | self . update_blocks (block)) ; buffer . digest_blocks (length_encode (self . customization . len () , & mut lenbuf) , | block | self . update_blocks (block) ,) ; if self . bufpos == CHUNK_SIZE && buffer . get_pos () != 0 { self . process_chunk () ; } self . buffer [self . bufpos .. (self . bufpos + buffer . get_pos ())] . copy_from_slice (buffer . get_data ()) ; self . bufpos += buffer . get_pos () ; if self . chain_length == 0 { let tshk = TurboShake128 :: < 0x07 > :: default () . chain (& self . buffer [.. self . bufpos]) . finalize_xof_reset () ; return KangarooTwelveReaderCore { tshk } ; } self . process_chaining_chunk () ; self . final_tshk . update (length_encode (self . chain_length , & mut lenbuf)) ; self . final_tshk . update (& [0xff , 0xff]) ; KangarooTwelveReaderCore { tshk : self . final_tshk . finalize_xof_reset () , } } }
};
}
