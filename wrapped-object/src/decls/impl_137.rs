macro_rules! deps {
    () => {
        ReadRef!();
        SectionIterator!();
        SectionIteratorInternal!();
        Item!();
        Section!();
        SectionInternal!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SectionIterator < 'data , 'file , R > { type Item = Section < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { next_inner ! (self . inner , SectionIteratorInternal , SectionInternal) . map (| inner | Section { inner }) } }
    };
}

impl_137!();