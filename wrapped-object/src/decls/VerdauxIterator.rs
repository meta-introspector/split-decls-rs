macro_rules! deps {
    () => {
        FileHeader!();
        Endian!();
        Bytes!();
    };
}

macro_rules! VerdauxIterator {
    () => {
        deps!();
        # [doc = " An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section."] # [derive (Debug , Clone)] pub struct VerdauxIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , count : u16 , }
    };
}

VerdauxIterator!();