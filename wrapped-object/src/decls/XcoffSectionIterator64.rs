macro_rules! deps {
    () => {
        XcoffSectionIterator!();
        FileHeader64!();
    };
}

macro_rules! XcoffSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSectionIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSectionIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSectionIterator64!()