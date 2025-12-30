// Generated macro for gz_init (function)
macro_rules! Depcrate_gzgz_init {
() => {
// Module: crate::gz
// Provides: {"gz_init"}
// Dependencies: {}
fn gz_init (state : & mut GzState) -> Result < () , () > { let capacity = state . in_capacity () ; state . in_size = capacity / 2 ; let Some (input) = ALLOCATOR . allocate_slice_raw :: < u8 > (capacity) else { unsafe { gz_error (state , Some ((Z_MEM_ERROR , "out of memory"))) } ; return Err (()) ; } ; state . input = input . as_ptr () ; if ! state . direct { let capacity = state . out_capacity () ; state . out_size = capacity ; let Some (output) = ALLOCATOR . allocate_slice_raw :: < u8 > (capacity) else { unsafe { free_buffers (state) } ; unsafe { gz_error (state , Some ((Z_MEM_ERROR , "out of memory"))) } ; return Err (()) ; } ; state . output = output . as_ptr () ; state . stream . zalloc = Some (ALLOCATOR . zalloc) ; state . stream . zfree = Some (ALLOCATOR . zfree) ; state . stream . opaque = ALLOCATOR . opaque ; const DEF_MEM_LEVEL : c_int = 8 ; if unsafe { deflateInit2_ (& mut state . stream , state . level as _ , Z_DEFLATED , MAX_WBITS + 16 , DEF_MEM_LEVEL , state . strategy as _ , zlibVersion () , core :: mem :: size_of :: < z_stream > () as _ ,) } != Z_OK { unsafe { free_buffers (state) } ; unsafe { gz_error (state , Some ((Z_MEM_ERROR , "out of memory"))) } ; return Err (()) ; } state . stream . next_in = ptr :: null_mut () ; } if ! state . direct { state . stream . avail_out = state . out_size as _ ; state . stream . next_out = state . output ; state . next = state . stream . next_out ; } Ok (()) }
};
}
