macro_rules! USIZE_BYTES {
    () => {
        # [doc = " The number of bytes in a single `usize` value."] const USIZE_BYTES : usize = (usize :: BITS / 8) as usize ;
    };
}

USIZE_BYTES!();