macro_rules! deps {
    () => {
        SectionIteratorInternal!();
        ReadRef!();
        File!();
    };
}

macro_rules! SectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`File`]."] # [derive (Debug)] pub struct SectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SectionIteratorInternal < 'data , 'file , R > , }
    };
}

SectionIterator!()