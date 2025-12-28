macro_rules! deps {
    () => {
        ReadRef!();
        Item!();
        MachOComdatSectionIterator!();
        MachHeader!();
        SectionIndex!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > Iterator for MachOComdatSectionIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_545!()