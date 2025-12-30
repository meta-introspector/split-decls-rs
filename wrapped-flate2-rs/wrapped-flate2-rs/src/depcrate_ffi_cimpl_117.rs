// Generated macro for impl_117 (impl)
macro_rules! Depcrate_ffi_cimpl_117 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_117"}
// Dependencies: {}
impl Inflate { unsafe fn decompress_inner (& mut self , input : & [u8] , output_ptr : * mut u8 , output_len : usize , flush : FlushDecompress ,) -> Result < Status , DecompressError > { let raw = self . inner . stream_wrapper . inner ; unsafe { (* raw) . msg = ptr :: null_mut () ; (* raw) . next_in = input . as_ptr () as * mut u8 ; (* raw) . avail_in = input . len () . min (c_uint :: MAX as usize) as c_uint ; (* raw) . next_out = output_ptr ; (* raw) . avail_out = output_len . min (c_uint :: MAX as usize) as c_uint ; let rc = mz_inflate (raw , flush as c_int) ; self . inner . total_in += ((* raw) . next_in as usize - input . as_ptr () as usize) as u64 ; self . inner . total_out += ((* raw) . next_out as usize - output_ptr as usize) as u64 ; (* raw) . next_in = ptr :: null_mut () ; (* raw) . avail_in = 0 ; (* raw) . next_out = ptr :: null_mut () ; (* raw) . avail_out = 0 ; match rc { MZ_DATA_ERROR | MZ_STREAM_ERROR | MZ_MEM_ERROR => { mem :: decompress_failed (self . inner . msg ()) } MZ_OK => Ok (Status :: Ok) , MZ_BUF_ERROR => Ok (Status :: BufError) , MZ_STREAM_END => Ok (Status :: StreamEnd) , # [allow (clippy :: unnecessary_cast)] MZ_NEED_DICT => mem :: decompress_need_dict ((* raw) . adler as u32) , c => panic ! ("unknown return code: {}" , c) , } } } }
};
}
