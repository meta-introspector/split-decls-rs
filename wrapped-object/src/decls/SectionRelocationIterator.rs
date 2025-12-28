macro_rules! deps {
    () => {
        SectionRelocationIteratorInternal!();
        Section!();
        ReadRef!();
    };
}

macro_rules! SectionRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the relocation entries in a [`Section`]."] # [derive (Debug)] pub struct SectionRelocationIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SectionRelocationIteratorInternal < 'data , 'file , R > , }
    };
}

SectionRelocationIterator!()