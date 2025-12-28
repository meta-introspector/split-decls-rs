macro_rules! deps {
    () => {
        MachOComdatIterator!();
        Item!();
        MachOComdat!();
        ReadRef!();
        MachHeader!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > Iterator for MachOComdatIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOComdat < 'data , 'file , Mach , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_536!()