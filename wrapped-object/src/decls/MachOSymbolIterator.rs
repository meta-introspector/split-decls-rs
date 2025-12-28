macro_rules! deps {
    () => {
        ReadRef!();
        MachOFile!();
        MachHeader!();
        SymbolIndex!();
    };
}

macro_rules! MachOSymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in a [`MachOFile`]."] pub struct MachOSymbolIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { file : & 'file MachOFile < 'data , Mach , R > , index : SymbolIndex , }
    };
}

MachOSymbolIterator!();