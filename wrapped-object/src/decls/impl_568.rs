macro_rules! deps {
    () => {
        Item!();
        MachOSegment!();
        MachOSegmentIterator!();
        ReadRef!();
        MachHeader!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > Iterator for MachOSegmentIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOSegment < 'data , 'file , Mach , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| internal | MachOSegment { file : self . file , internal , }) } }
    };
}

impl_568!();