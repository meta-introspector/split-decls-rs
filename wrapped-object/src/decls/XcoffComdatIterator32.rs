macro_rules! deps {
    () => {
        XcoffComdatIterator!();
        FileHeader32!();
    };
}

macro_rules! XcoffComdatIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffComdatIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffComdatIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffComdatIterator32!()