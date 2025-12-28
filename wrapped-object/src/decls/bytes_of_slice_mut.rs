macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! bytes_of_slice_mut {
    () => {
        deps!();
        # [doc = " Cast a slice of a `Pod` type to a mutable byte slice."] # [inline] pub fn bytes_of_slice_mut < T : Pod > (val : & mut [T]) -> & mut [u8] { let size = val . len () . wrapping_mul (mem :: size_of :: < T > ()) ; unsafe { slice :: from_raw_parts_mut (val . as_mut_ptr () . cast () , size) } }
    };
}

bytes_of_slice_mut!();