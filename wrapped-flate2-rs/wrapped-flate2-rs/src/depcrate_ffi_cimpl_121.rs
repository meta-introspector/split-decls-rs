// Generated macro for impl_121 (impl)
macro_rules! Depcrate_ffi_cimpl_121 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_121"}
// Dependencies: {}
impl Deflate { unsafe fn compress_inner (& mut self , input : & [u8] , output_ptr : * mut u8 , output_len : usize , flush : FlushCompress ,) -> Result < Status , CompressError > { let raw = self . inner . stream_wrapper . inner ; unsafe { (* raw) . msg = ptr :: null_mut () ; (* raw) . next_in = input . as_ptr () as * mut _ ; (* raw) . avail_in = input . len () . min (c_uint :: MAX as usize) as c_uint ; (* raw) . next_out = output_ptr ; (* raw) . avail_out = output_len . min (c_uint :: MAX as usize) as c_uint ; let rc = mz_deflate (raw , flush as c_int) ; self . inner . total_in += ((* raw) . next_in as usize - input . as_ptr () as usize) as u64 ; self . inner . total_out += ((* raw) . next_out as usize - output_ptr as usize) as u64 ; (* raw) . next_in = ptr :: null_mut () ; (* raw) . avail_in = 0 ; (* raw) . next_out = ptr :: null_mut () ; (* raw) . avail_out = 0 ; match rc { MZ_OK => Ok (Status :: Ok) , MZ_BUF_ERROR => Ok (Status :: BufError) , MZ_STREAM_END => Ok (Status :: StreamEnd) , MZ_STREAM_ERROR => mem :: compress_failed (self . inner . msg ()) , c => panic ! ("unknown return code: {}" , c) , } } } }
};
}
