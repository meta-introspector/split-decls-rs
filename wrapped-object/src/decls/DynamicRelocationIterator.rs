macro_rules! deps {
    () => {
        DynamicRelocationIteratorInternal!();
        ReadRef!();
        File!();
    };
}

macro_rules! DynamicRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the dynamic relocation entries in a [`File`]."] # [derive (Debug)] pub struct DynamicRelocationIterator < 'data , 'file , R = & 'data [u8] > where R : ReadRef < 'data > , { inner : DynamicRelocationIteratorInternal < 'data , 'file , R > , }
    };
}

DynamicRelocationIterator!();