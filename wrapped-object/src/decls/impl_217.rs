macro_rules! deps {
    () => {
        SectionIndex!();
        CoffHeader!();
        CoffSectionIterator!();
        CoffSection!();
        Item!();
        ReadRef!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffSectionIterator < 'data , 'file , R , Coff > { type Item = CoffSection < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | CoffSection { file : self . file , index : SectionIndex (index + 1) , section , }) } }
    };
}

impl_217!()