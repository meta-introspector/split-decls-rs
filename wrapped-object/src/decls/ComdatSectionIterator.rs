macro_rules! deps {
    () => {
        ReadRef!();
        ComdatSectionIteratorInternal!();
        Comdat!();
    };
}

macro_rules! ComdatSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`Comdat`]."] # [derive (Debug)] pub struct ComdatSectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : ComdatSectionIteratorInternal < 'data , 'file , R > , }
    };
}

ComdatSectionIterator!()