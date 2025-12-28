macro_rules! deps {
    () => {
        FileHeader64!();
        XcoffComdatIterator!();
    };
}

macro_rules! XcoffComdatIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffComdatIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffComdatIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffComdatIterator64!()