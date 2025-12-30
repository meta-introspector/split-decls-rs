// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl VariableOutputCore for KupynaShortVarCore { const TRUNC_SIDE : TruncSide = TruncSide :: Right ; # [inline] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > { if output_size > Self :: OutputSize :: USIZE { return Err (InvalidOutputSize) ; } let mut state = [0 ; short :: COLS] ; state [0] = 0x40 ; state [0] <<= 56 ; let blocks_len = 0 ; Ok (Self { state , blocks_len }) } # [inline] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let block_size = Self :: BlockSize :: USIZE as u128 ; let msg_len_bytes = (self . blocks_len as u128) * block_size + (buffer . get_pos () as u128) ; let msg_len_bits = 8 * msg_len_bytes ; buffer . digest_pad (0x80 , & msg_len_bits . to_le_bytes () [0 .. 12] , | block | { short :: compress (& mut self . state , block . as_ref ()) ; }) ; let t_xor_ult_processed_block = short :: t_xor_l (self . state) ; let result_state = xor (self . state , t_xor_ult_processed_block) ; write_u64_be (& result_state [short :: COLS / 2 ..] , out) ; } }
};
}
