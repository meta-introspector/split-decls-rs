// Generated macro for impl_49 (impl)
macro_rules! Depcrate_reservedimpl_49 {
() => {
// Module: crate::reserved
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'inp , 'out , T > InOutBufReserved < 'inp , 'out , T > { # [doc = " Crate [`InOutBufReserved`] from two separate slices."] pub fn from_slices (in_buf : & 'inp [T] , out_buf : & 'out mut [T] ,) -> Result < Self , OutIsTooSmallError > { if in_buf . len () > out_buf . len () { return Err (OutIsTooSmallError) ; } Ok (Self { in_ptr : in_buf . as_ptr () , out_ptr : out_buf . as_mut_ptr () , in_len : in_buf . len () , out_len : out_buf . len () , _pd : PhantomData , }) } # [doc = " Get input slice."] # [inline (always)] pub fn get_in (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . in_ptr , self . in_len) } } # [doc = " Get output slice."] # [inline (always)] pub fn get_out (& mut self) -> & mut [T] { unsafe { slice :: from_raw_parts_mut (self . out_ptr , self . out_len) } } # [doc = " Consume `self` and get output slice with lifetime `'out`."] # [inline (always)] pub fn into_out (self) -> & 'out mut [T] { unsafe { slice :: from_raw_parts_mut (self . out_ptr , self . out_len) } } }
};
}
