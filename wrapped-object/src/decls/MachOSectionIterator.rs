macro_rules! deps {
    () => {
        MachOSectionInternal!();
        MachHeader!();
        ReadRef!();
        MachOFile!();
    };
}

macro_rules! MachOSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`MachOFile`]."] pub struct MachOSectionIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) iter : slice :: Iter < 'file , MachOSectionInternal < 'data , Mach , R > > , }
    };
}

MachOSectionIterator!()