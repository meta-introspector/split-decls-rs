macro_rules! deps {
    () => {
        PeFile!();
        ImageNtHeaders64!();
    };
}

macro_rules! PeFile64 {
    () => {
        deps!();
        # [doc = " A PE32+ (64-bit) image file."] # [doc = ""] # [doc = " This is a file that starts with [`pe::ImageNtHeaders64`], and corresponds"] # [doc = " to [`crate::FileKind::Pe64`]."] pub type PeFile64 < 'data , R = & 'data [u8] > = PeFile < 'data , pe :: ImageNtHeaders64 , R > ;
    };
}

PeFile64!();