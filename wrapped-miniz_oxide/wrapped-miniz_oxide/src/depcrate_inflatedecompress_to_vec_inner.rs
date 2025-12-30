// Generated macro for decompress_to_vec_inner (function)
macro_rules! Depcrate_inflatedecompress_to_vec_inner {
() => {
// Module: crate::inflate
// Provides: {"decompress_to_vec_inner"}
// Dependencies: {}
# [doc = " Backend of various to-[`Vec`] decompressions."] # [doc = ""] # [doc = " Returns [`Vec`] of decompressed data on success and the [error struct][DecompressError] with details on failure."] # [cfg (feature = "with-alloc")] fn decompress_to_vec_inner (mut input : & [u8] , flags : u32 , max_output_size : usize ,) -> Result < Vec < u8 > , DecompressError > { let flags = flags | inflate_flags :: TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF ; let mut ret : Vec < u8 > = vec ! [0 ; input . len () . saturating_mul (2) . min (max_output_size)] ; let mut decomp = Box :: < DecompressorOxide > :: default () ; let mut out_pos = 0 ; loop { let (status , in_consumed , out_consumed) = decompress (& mut decomp , input , & mut ret , out_pos , flags) ; out_pos += out_consumed ; match status { TINFLStatus :: Done => { ret . truncate (out_pos) ; return Ok (ret) ; } TINFLStatus :: HasMoreOutput => { if in_consumed > input . len () { return decompress_error (TINFLStatus :: HasMoreOutput , ret) ; } input = & input [in_consumed ..] ; if ret . len () >= max_output_size { return decompress_error (TINFLStatus :: HasMoreOutput , ret) ; } let new_len = ret . len () . saturating_mul (2) . min (max_output_size) ; ret . resize (new_len , 0) ; } _ => return decompress_error (status , ret) , } } }
};
}
