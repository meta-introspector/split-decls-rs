macro_rules! deps {
    () => {
        ReadRef!();
        ComdatIteratorInternal!();
        File!();
    };
}

macro_rules! ComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`File`]."] # [derive (Debug)] pub struct ComdatIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : ComdatIteratorInternal < 'data , 'file , R > , }
    };
}

ComdatIterator!();