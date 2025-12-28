macro_rules! deps {
    () => {
        ReadRef!();
        MachOSection!();
        Item!();
        MachHeader!();
        MachOSectionIterator!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > Iterator for MachOSectionIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOSection < 'data , 'file , Mach , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| & internal | MachOSection { file : self . file , internal , }) } }
    };
}

impl_584!();