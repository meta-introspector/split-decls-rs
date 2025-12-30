// Generated macro for impl_38 (impl)
macro_rules! Depcrate_gutsimpl_38 {
() => {
// Module: crate::guts
// Provides: {"impl_38"}
// Dependencies: {}
impl ChaCha { # [inline (always)] pub fn new (key : & [u8 ; 32] , nonce : & [u8]) -> Self { init_chacha (key , nonce) } # [doc = " Produce 4 blocks of output, advancing the state"] # [inline (always)] pub fn refill4 (& mut self , drounds : u32 , out : & mut [u32 ; BUFSZ]) { refill_wide (self , drounds , out) } # [inline (always)] pub fn set_block_pos (& mut self , value : u64) { set_stream_param (self , STREAM_PARAM_BLOCK , value) } # [inline (always)] pub fn get_block_pos (& self) -> u64 { get_stream_param (self , STREAM_PARAM_BLOCK) } # [inline (always)] pub fn set_nonce (& mut self , value : u64) { set_stream_param (self , STREAM_PARAM_NONCE , value) } # [inline (always)] pub fn get_nonce (& self) -> u64 { get_stream_param (self , STREAM_PARAM_NONCE) } # [inline (always)] pub fn get_seed (& self) -> [u8 ; 32] { get_seed (self) } }
};
}
