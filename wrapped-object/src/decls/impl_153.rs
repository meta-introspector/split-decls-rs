macro_rules! deps {
    () => {
        ComdatSectionIterator!();
        SectionIndex!();
        ReadRef!();
        Item!();
        ComdatSectionIteratorInternal!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for ComdatSectionIterator < 'data , 'file , R > { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { with_inner_mut ! (self . inner , ComdatSectionIteratorInternal , | x | x . next ()) } }
    };
}

impl_153!();