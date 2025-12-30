// Generated macro for impl_26 (impl)
macro_rules! Depcrate_bioimpl_26 {
() => {
// Module: crate::bio
// Provides: {"impl_26"}
// Dependencies: {}
impl MemBio { pub fn new () -> Result < MemBio , ErrorStack > { ffi :: init () ; let bio = unsafe { cvt_p (ffi :: BIO_new (ffi :: BIO_s_mem ())) ? } ; Ok (MemBio (bio)) } pub fn as_ptr (& self) -> * mut ffi :: BIO { self . 0 } pub fn get_buf (& self) -> & [u8] { unsafe { let mut ptr = ptr :: null_mut () ; let len = ffi :: BIO_get_mem_data (self . 0 , & mut ptr) ; util :: from_raw_parts (ptr as * const _ as * const _ , len as usize) } } # [cfg (not (any (boringssl , awslc)))] pub unsafe fn from_ptr (bio : * mut ffi :: BIO) -> MemBio { MemBio (bio) } }
};
}
