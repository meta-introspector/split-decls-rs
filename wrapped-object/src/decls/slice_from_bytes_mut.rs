macro_rules! deps {
    () => {
        Pod!();
        Result!();
    };
}

macro_rules! slice_from_bytes_mut {
    () => {
        deps!();
        # [doc = " Cast the head of a mutable byte slice to a slice of a `Pod` type."] # [doc = ""] # [doc = " Returns the type slice and the tail of the byte slice."] # [doc = ""] # [doc = " Returns an error if the byte slice is too short or the alignment is invalid."] # [inline] pub fn slice_from_bytes_mut < T : Pod > (data : & mut [u8] , count : usize ,) -> Result < (& mut [T] , & mut [u8]) > { let size = count . checked_mul (mem :: size_of :: < T > ()) . ok_or (()) ? ; if size > data . len () { return Err (()) ; } let (data , tail) = data . split_at_mut (size) ; let ptr = data . as_mut_ptr () ; if (ptr as usize) % mem :: align_of :: < T > () != 0 { return Err (()) ; } let slice = unsafe { slice :: from_raw_parts_mut (ptr . cast () , count) } ; Ok ((slice , tail)) }
    };
}

slice_from_bytes_mut!()