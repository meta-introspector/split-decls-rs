macro_rules! deps {
    () => {
        MachOSectionInternal!();
        MachOFile!();
        ReadRef!();
        MachHeader!();
        ObjectSection!();
    };
}

macro_rules! MachOSection {
    () => {
        deps!();
        # [doc = " A section in a [`MachOFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct MachOSection < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) internal : MachOSectionInternal < 'data , Mach , R > , }
    };
}

MachOSection!()