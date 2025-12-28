macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! bytes_of_slice {
    () => {
        deps!();
        # [doc = " Cast a slice of a `Pod` type to a byte slice."] # [inline] pub fn bytes_of_slice < T : Pod > (val : & [T]) -> & [u8] { let size = val . len () . wrapping_mul (mem :: size_of :: < T > ()) ; unsafe { slice :: from_raw_parts (val . as_ptr () . cast () , size) } }
    };
}

bytes_of_slice!();