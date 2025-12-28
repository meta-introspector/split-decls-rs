macro_rules! deps {
    () => {
        Endian!();
        ElfFile!();
        Endianness!();
        FileHeader64!();
    };
}

macro_rules! ElfFile64 {
    () => {
        deps!();
        # [doc = " A 64-bit ELF object file."] # [doc = ""] # [doc = " This is a file that starts with [`elf::FileHeader64`], and corresponds"] # [doc = " to [`crate::FileKind::Elf64`]."] pub type ElfFile64 < 'data , Endian = Endianness , R = & 'data [u8] > = ElfFile < 'data , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfFile64!()