macro_rules! deps {
    () => {
        FileHeader!();
        Endian!();
        Bytes!();
    };
}

macro_rules! VerneedIterator {
    () => {
        deps!();
        # [doc = " An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section."] # [derive (Debug , Clone)] pub struct VerneedIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
    };
}

VerneedIterator!();