// Generated macro for impl_10 (impl)
macro_rules! Depcrate_block_apiimpl_10 {
() => {
// Module: crate::block_api
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'cs > KangarooTwelveCore < 'cs > { # [doc = " Creates a new KangarooTwelve instance with the given customization."] pub fn new (customization : & 'cs [u8]) -> Self { Self { customization , buffer : [0u8 ; CHUNK_SIZE] , bufpos : 0usize , final_tshk : Default :: default () , chain_tshk : Default :: default () , chain_length : 0usize , } } fn process_chunk (& mut self) { debug_assert ! (self . bufpos == CHUNK_SIZE) ; if self . chain_length == 0 { self . final_tshk . update (& self . buffer) ; } else { self . process_chaining_chunk () ; } self . chain_length += 1 ; self . buffer = [0u8 ; CHUNK_SIZE] ; self . bufpos = 0 ; } fn process_chaining_chunk (& mut self) { debug_assert ! (self . bufpos != 0) ; if self . chain_length == 1 { self . final_tshk . update (& [0x03 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00]) ; } let mut result = [0u8 ; CHAINING_VALUE_SIZE] ; self . chain_tshk . update (& self . buffer [.. self . bufpos]) ; self . chain_tshk . finalize_xof_reset_into (& mut result) ; self . final_tshk . update (& result) ; } }
};
}
