// Generated macro for impl_533 (impl)
macro_rules! Depcrate_cbbimpl_533 {
() => {
// Module: crate::cbb
// Provides: {"impl_533"}
// Dependencies: {}
impl LcCBB < 'static > { pub (crate) fn new (initial_capacity : usize) -> LcCBB < 'static > { let mut cbb = MaybeUninit :: < CBB > :: uninit () ; let cbb = unsafe { CBB_init (cbb . as_mut_ptr () , initial_capacity) ; cbb . assume_init () } ; Self (cbb , PhantomData) } pub (crate) fn into_vec (mut self) -> Result < Vec < u8 > , Unspecified > { let mut out_data = null_mut :: < u8 > () ; let mut out_len : usize = 0 ; if 1 != unsafe { CBB_finish (self . as_mut_ptr () , & mut out_data , & mut out_len) } { return Err (Unspecified) ; } let out_data = LcPtr :: new (out_data) ? ; let slice = unsafe { std :: slice :: from_raw_parts (* out_data . as_const () , out_len) } ; Ok (slice . to_vec ()) } }
};
}
