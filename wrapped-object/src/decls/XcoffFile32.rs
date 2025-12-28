macro_rules! deps {
    () => {
        XcoffFile!();
        FileHeader32!();
    };
}

macro_rules! XcoffFile32 {
    () => {
        deps!();
        # [doc = " A 32-bit XCOFF object file."] # [doc = ""] # [doc = " This is a file that starts with [`xcoff::FileHeader32`], and corresponds"] # [doc = " to [`crate::FileKind::Xcoff32`]."] pub type XcoffFile32 < 'data , R = & 'data [u8] > = XcoffFile < 'data , xcoff :: FileHeader32 , R > ;
    };
}

XcoffFile32!();