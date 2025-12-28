macro_rules! deps {
    () => {
        Result!();
        Pod!();
    };
}

macro_rules! slice_from_all_bytes {
    () => {
        deps!();
        # [doc = " Cast all of a byte slice to a slice of a `Pod` type."] # [doc = ""] # [doc = " Returns the type slice."] # [doc = ""] # [doc = " Returns an error if the size of the byte slice is not an exact multiple"] # [doc = " of the type size, or the alignment is invalid."] # [inline] pub fn slice_from_all_bytes < T : Pod > (data : & [u8]) -> Result < & [T] > { let count = data . len () / mem :: size_of :: < T > () ; let (slice , tail) = slice_from_bytes (data , count) ? ; if ! tail . is_empty () { return Err (()) ; } Ok (slice) }
    };
}

slice_from_all_bytes!();