macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! bytes_of {
    () => {
        deps!();
        # [doc = " Cast a `Pod` type to a byte slice."] # [inline] pub fn bytes_of < T : Pod > (val : & T) -> & [u8] { let size = mem :: size_of :: < T > () ; unsafe { slice :: from_raw_parts (slice :: from_ref (val) . as_ptr () . cast () , size) } }
    };
}

bytes_of!();