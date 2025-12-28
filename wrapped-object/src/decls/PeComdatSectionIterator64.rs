macro_rules! deps {
    () => {
        ImageNtHeaders64!();
        PeFile64!();
        PeComdatSectionIterator!();
    };
}

macro_rules! PeComdatSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`PeFile64`]."] pub type PeComdatSectionIterator64 < 'data , 'file , R = & 'data [u8] > = PeComdatSectionIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeComdatSectionIterator64!()