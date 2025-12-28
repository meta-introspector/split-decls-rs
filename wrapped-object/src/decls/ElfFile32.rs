macro_rules! deps {
    () => {
        ElfFile!();
        Endianness!();
        FileHeader32!();
        Endian!();
    };
}

macro_rules! ElfFile32 {
    () => {
        deps!();
        # [doc = " A 32-bit ELF object file."] # [doc = ""] # [doc = " This is a file that starts with [`elf::FileHeader32`], and corresponds"] # [doc = " to [`crate::FileKind::Elf32`]."] pub type ElfFile32 < 'data , Endian = Endianness , R = & 'data [u8] > = ElfFile < 'data , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfFile32!()