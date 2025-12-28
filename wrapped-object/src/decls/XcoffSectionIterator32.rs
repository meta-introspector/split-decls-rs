macro_rules! deps {
    () => {
        FileHeader32!();
        XcoffSectionIterator!();
    };
}

macro_rules! XcoffSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSectionIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffSectionIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSectionIterator32!()