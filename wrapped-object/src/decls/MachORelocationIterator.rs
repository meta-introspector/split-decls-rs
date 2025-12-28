macro_rules! deps {
    () => {
        Endian!();
        ReadRef!();
        MachOFile!();
        MachHeader!();
        Relocation!();
    };
}

macro_rules! MachORelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in a [`MachOSection`](super::MachOSection)."] pub struct MachORelocationIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) relocations : slice :: Iter < 'data , macho :: Relocation < Mach :: Endian > > , }
    };
}

MachORelocationIterator!();