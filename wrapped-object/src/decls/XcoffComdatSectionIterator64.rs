macro_rules! deps {
    () => {
        XcoffComdatSectionIterator!();
        FileHeader64!();
    };
}

macro_rules! XcoffComdatSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffComdatSectionIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffComdatSectionIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffComdatSectionIterator64!()