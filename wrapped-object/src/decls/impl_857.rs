macro_rules! deps {
    () => {
        Item!();
        XcoffComdatSectionIterator!();
        SectionIndex!();
        ReadRef!();
        FileHeader!();
    };
}

macro_rules! impl_857 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > Iterator for XcoffComdatSectionIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_857!();