macro_rules! deps {
    () => {
        MachOFile!();
        ReadRef!();
        MachHeader!();
    };
}

macro_rules! MachOComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`MachOFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct MachOComdatIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file MachOFile < 'data , Mach , R > , }
    };
}

MachOComdatIterator!();