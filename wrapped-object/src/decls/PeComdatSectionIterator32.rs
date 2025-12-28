macro_rules! deps {
    () => {
        PeFile32!();
        PeComdatSectionIterator!();
        ImageNtHeaders32!();
    };
}

macro_rules! PeComdatSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`PeFile32`]."] pub type PeComdatSectionIterator32 < 'data , 'file , R = & 'data [u8] > = PeComdatSectionIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeComdatSectionIterator32!();