macro_rules! deps {
    () => {
        FileHeader32!();
        XcoffComdatIterator!();
    };
}

macro_rules! XcoffComdatIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffComdatIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffComdatIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffComdatIterator32!();