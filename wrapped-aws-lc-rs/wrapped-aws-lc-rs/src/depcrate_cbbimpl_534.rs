// Generated macro for impl_534 (impl)
macro_rules! Depcrate_cbbimpl_534 {
() => {
// Module: crate::cbb
// Provides: {"impl_534"}
// Dependencies: {}
impl < 'a > LcCBB < 'a > { pub (crate) fn new_from_slice (buffer : & 'a mut [u8]) -> LcCBB < 'a > { let mut cbb = MaybeUninit :: < CBB > :: uninit () ; let cbb = unsafe { CBB_init_fixed (cbb . as_mut_ptr () , buffer . as_mut_ptr () , buffer . len ()) ; cbb . assume_init () } ; Self (cbb , PhantomData) } pub (crate) fn finish (mut self) -> Result < usize , Unspecified > { let mut pkcs8_bytes_ptr = null_mut :: < u8 > () ; let mut out_len : usize = 0 ; if 1 != unsafe { CBB_finish (self . as_mut_ptr () , & mut pkcs8_bytes_ptr , & mut out_len) } { return Err (Unspecified) ; } Ok (out_len) } }
};
}
