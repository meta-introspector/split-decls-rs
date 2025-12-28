macro_rules! deps {
    () => {
        XcoffRelocationIterator!();
        FileHeader32!();
    };
}

macro_rules! XcoffRelocationIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32)."] pub type XcoffRelocationIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffRelocationIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffRelocationIterator32!();