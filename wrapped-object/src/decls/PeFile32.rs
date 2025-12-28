macro_rules! deps {
    () => {
        PeFile!();
        ImageNtHeaders32!();
    };
}

macro_rules! PeFile32 {
    () => {
        deps!();
        # [doc = " A PE32 (32-bit) image file."] # [doc = ""] # [doc = " This is a file that starts with [`pe::ImageNtHeaders32`], and corresponds"] # [doc = " to [`crate::FileKind::Pe32`]."] pub type PeFile32 < 'data , R = & 'data [u8] > = PeFile < 'data , pe :: ImageNtHeaders32 , R > ;
    };
}

PeFile32!()