macro_rules! deps {
    () => {
        FileHeader64!();
        XcoffRelocationIterator!();
    };
}

macro_rules! XcoffRelocationIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64)."] pub type XcoffRelocationIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffRelocationIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffRelocationIterator64!()