macro_rules! deps {
    () => {
        XcoffSection!();
        ReadRef!();
        FileHeader!();
        SectionIndex!();
        Item!();
        XcoffSectionIterator!();
    };
}

macro_rules! impl_791 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > Iterator for XcoffSectionIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = XcoffSection < 'data , 'file , Xcoff , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | XcoffSection { index : SectionIndex (index + 1) , file : self . file , section , }) } }
    };
}

impl_791!();