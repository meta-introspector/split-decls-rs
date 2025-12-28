macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! MAGIC {
    () => {
        deps!();
        # [doc = " File identification bytes stored at the beginning of the file."] pub const MAGIC : [u8 ; 8] = * b"!<arch>\n" ;
    };
}

MAGIC!();