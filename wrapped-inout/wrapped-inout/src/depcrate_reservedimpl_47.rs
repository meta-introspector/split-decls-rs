// Generated macro for impl_47 (impl)
macro_rules! Depcrate_reservedimpl_47 {
() => {
// Module: crate::reserved
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , T > InOutBufReserved < 'a , 'a , T > { # [doc = " Crate [`InOutBufReserved`] from a single mutable slice."] pub fn from_mut_slice (buf : & 'a mut [T] , msg_len : usize) -> Result < Self , OutIsTooSmallError > { if msg_len > buf . len () { return Err (OutIsTooSmallError) ; } let p = buf . as_mut_ptr () ; let out_len = buf . len () ; Ok (Self { in_ptr : p , out_ptr : p , in_len : msg_len , out_len , _pd : PhantomData , }) } }
};
}
