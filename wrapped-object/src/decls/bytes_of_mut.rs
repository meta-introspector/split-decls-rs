macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! bytes_of_mut {
    () => {
        deps!();
        # [doc = " Cast a `Pod` type to a mutable byte slice."] # [inline] pub fn bytes_of_mut < T : Pod > (val : & mut T) -> & mut [u8] { let size = mem :: size_of :: < T > () ; unsafe { slice :: from_raw_parts_mut (slice :: from_mut (val) . as_mut_ptr () . cast () , size) } }
    };
}

bytes_of_mut!();