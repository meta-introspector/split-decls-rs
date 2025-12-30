// Generated macro for impl_19 (impl)
macro_rules! Depcrate_bitmapimpl_19 {
() => {
// Module: crate::bitmap
// Provides: {"impl_19"}
// Dependencies: {}
impl < const SIZE : usize > TryFrom < & [u8] > for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { type Error = () ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { if value . len () == size_of :: < < BitsImpl < SIZE > as Bits > :: Store > () { let mut data : MaybeUninit < < BitsImpl < SIZE > as Bits > :: Store > = MaybeUninit :: uninit () ; let data_ptr : * mut u8 = data . as_mut_ptr () . cast () ; Ok (unsafe { data_ptr . copy_from_nonoverlapping (value . as_ptr () , value . len ()) ; Self { data : data . assume_init () , } }) } else { Err (()) } } }
};
}
