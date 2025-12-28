macro_rules! deps {
    () => {
        Relocation!();
        SectionRelocationIteratorInternal!();
        SectionRelocationIterator!();
        Item!();
        ReadRef!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SectionRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { with_inner_mut ! (self . inner , SectionRelocationIteratorInternal , | x | x . next ()) } }
    };
}

impl_171!()