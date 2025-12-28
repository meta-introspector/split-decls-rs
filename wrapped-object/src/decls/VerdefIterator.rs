macro_rules! deps {
    () => {
        Endian!();
        FileHeader!();
        Bytes!();
    };
}

macro_rules! VerdefIterator {
    () => {
        deps!();
        # [doc = " An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section."] # [derive (Debug , Clone)] pub struct VerdefIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
    };
}

VerdefIterator!()