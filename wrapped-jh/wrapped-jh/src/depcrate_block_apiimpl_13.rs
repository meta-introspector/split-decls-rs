// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl VariableOutputCore for JhCore { const TRUNC_SIDE : TruncSide = TruncSide :: Right ; # [inline] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > { let h0 = match output_size { 28 => consts :: JH224_H0 , 32 => consts :: JH256_H0 , 48 => consts :: JH384_H0 , 64 => consts :: JH512_H0 , _ => return Err (InvalidOutputSize) , } ; Ok (Self { state : Compressor :: new (h0) , block_len : 0 , }) } # [inline] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bit_len = self . block_len . wrapping_mul (Self :: BlockSize :: U64) . wrapping_add (buffer . get_pos () as u64) . wrapping_mul (8) ; if buffer . get_pos () == 0 { buffer . len64_padding_be (bit_len , | b | self . state . update (b)) ; } else { buffer . digest_pad (0x80 , & [] , | b | self . state . update (b)) ; buffer . digest_pad (0 , & bit_len . to_be_bytes () , | b | self . state . update (b)) ; } out . copy_from_slice (& self . state . finalize () [64 ..]) ; } }
};
}
