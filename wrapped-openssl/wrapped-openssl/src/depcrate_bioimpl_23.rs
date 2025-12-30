// Generated macro for impl_23 (impl)
macro_rules! Depcrate_bioimpl_23 {
() => {
// Module: crate::bio
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a > MemBioSlice < 'a > { pub fn new (buf : & 'a [u8]) -> Result < MemBioSlice < 'a > , ErrorStack > { ffi :: init () ; assert ! (buf . len () <= c_int :: MAX as usize) ; let bio = unsafe { cvt_p (ffi :: BIO_new_mem_buf (buf . as_ptr () as * const _ , buf . len () as crate :: SLenType ,)) ? } ; Ok (MemBioSlice (bio , PhantomData)) } pub fn as_ptr (& self) -> * mut ffi :: BIO { self . 0 } }
};
}
