macro_rules! deps {
    () => {
        XcoffFile!();
        FileHeader64!();
    };
}

macro_rules! XcoffFile64 {
    () => {
        deps!();
        # [doc = " A 64-bit XCOFF object file."] # [doc = ""] # [doc = " This is a file that starts with [`xcoff::FileHeader64`], and corresponds"] # [doc = " to [`crate::FileKind::Xcoff64`]."] pub type XcoffFile64 < 'data , R = & 'data [u8] > = XcoffFile < 'data , xcoff :: FileHeader64 , R > ;
    };
}

XcoffFile64!();