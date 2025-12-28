macro_rules! deps {
    () => {
        ImageNtHeaders32!();
        PeComdatIterator!();
        PeFile32!();
    };
}

macro_rules! PeComdatIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`PeFile32`]."] pub type PeComdatIterator32 < 'data , 'file , R = & 'data [u8] > = PeComdatIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeComdatIterator32!();