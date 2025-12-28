macro_rules! deps {
    () => {
        FileHeader!();
        Endian!();
        Bytes!();
    };
}

macro_rules! VernauxIterator {
    () => {
        deps!();
        # [doc = " An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section."] # [derive (Debug , Clone)] pub struct VernauxIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , count : u16 , }
    };
}

VernauxIterator!();