macro_rules! deps {
    () => {
        Item!();
        ComdatIterator!();
        Comdat!();
        ReadRef!();
        ComdatIteratorInternal!();
        ComdatInternal!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for ComdatIterator < 'data , 'file , R > { type Item = Comdat < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { next_inner ! (self . inner , ComdatIteratorInternal , ComdatInternal) . map (| inner | Comdat { inner }) } }
    };
}

impl_145!();