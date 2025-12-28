macro_rules! deps {
    () => {
        MachOFile!();
        MachHeader!();
        ReadRef!();
    };
}

macro_rules! MachOComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`MachOFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct MachOComdat < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file MachOFile < 'data , Mach , R > , }
    };
}

MachOComdat!();