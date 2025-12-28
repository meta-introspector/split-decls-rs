macro_rules! deps {
    () => {
        Pod!();
        Result!();
    };
}

macro_rules! from_bytes {
    () => {
        deps!();
        # [doc = " Cast the head of a byte slice to a `Pod` type."] # [doc = ""] # [doc = " Returns the type and the tail of the byte slice."] # [doc = ""] # [doc = " Returns an error if the byte slice is too short or the alignment is invalid."] # [inline] pub fn from_bytes < T : Pod > (data : & [u8]) -> Result < (& T , & [u8]) > { let size = mem :: size_of :: < T > () ; let tail = data . get (size ..) . ok_or (()) ? ; let ptr = data . as_ptr () ; if (ptr as usize) % mem :: align_of :: < T > () != 0 { return Err (()) ; } let val = unsafe { & * ptr . cast () } ; Ok ((val , tail)) }
    };
}

from_bytes!()