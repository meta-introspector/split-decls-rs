// Generated macro for write_callback (function)
macro_rules! Depcratewrite_callback {
() => {
// Module: crate
// Provides: {"write_callback"}
// Dependencies: {}
# [doc = " Callback function passed to `lprofWriteData`."] unsafe extern "C" fn write_callback < Writer : CoverageWriter > (this : * mut ProfDataWriter , iovecs : * mut ProfDataIOVec , num_iovecs : u32 ,) -> u32 { let writer = & mut * ((* this) . WriterCtx as * mut Writer) ; for iov in slice :: from_raw_parts (iovecs , num_iovecs as usize) { let len = iov . ElmSize * iov . NumElm ; if iov . Data . is_null () { let zero = [0 ; 16] ; let mut remaining = len ; while remaining != 0 { let data = & zero [.. usize :: min (zero . len () , remaining)] ; if writer . write (data) . is_err () { return 1 ; } remaining -= data . len () ; } } else { let data = slice :: from_raw_parts (iov . Data , len) ; if writer . write (data) . is_err () { return 1 ; } } } 0 }
};
}
