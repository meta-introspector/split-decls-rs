macro_rules! deps {
    () => {
        XcoffComdatSectionIterator!();
        FileHeader32!();
    };
}

macro_rules! XcoffComdatSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffComdatSectionIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffComdatSectionIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffComdatSectionIterator32!()