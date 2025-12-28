macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! AIX_BIG_MAGIC {
    () => {
        deps!();
        # [doc = " File identification bytes at the beginning of AIX big archive."] pub const AIX_BIG_MAGIC : [u8 ; 8] = * b"<bigaf>\n" ;
    };
}

AIX_BIG_MAGIC!()