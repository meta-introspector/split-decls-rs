macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! THIN_MAGIC {
    () => {
        deps!();
        # [doc = " File identification bytes stored at the beginning of a thin archive."] # [doc = ""] # [doc = " A thin archive only contains a symbol table and file names."] pub const THIN_MAGIC : [u8 ; 8] = * b"!<thin>\n" ;
    };
}

THIN_MAGIC!()