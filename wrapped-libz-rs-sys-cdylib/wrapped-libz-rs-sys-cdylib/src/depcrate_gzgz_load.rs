// Generated macro for gz_load (function)
macro_rules! Depcrate_gzgz_load {
() => {
// Module: crate::gz
// Provides: {"gz_load"}
// Dependencies: {}
unsafe fn gz_load (state : & mut GzState , buf : * mut u8 , len : usize) -> Result < usize , () > { let mut have = 0 ; let mut ret = 0 ; while have < len { ret = unsafe { libc :: read (state . fd , buf . add (have) . cast :: < _ > () , (len - have) as _) } ; if ret <= 0 { break ; } have += ret as usize ; } if ret < 0 { unsafe { gz_error (state , Some ((Z_ERRNO , "read error"))) } ; return Err (()) ; } if ret == 0 { state . eof = true ; } Ok (have) }
};
}
