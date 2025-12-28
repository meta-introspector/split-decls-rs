macro_rules! deps {
    () => {
        ImageNtHeaders32!();
        PeSectionIterator!();
    };
}

macro_rules! PeSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`PeFile32`](super::PeFile32)."] pub type PeSectionIterator32 < 'data , 'file , R = & 'data [u8] > = PeSectionIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeSectionIterator32!()