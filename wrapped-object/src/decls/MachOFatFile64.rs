macro_rules! deps {
    () => {
        MachOFatFile!();
        FatArch64!();
    };
}

macro_rules! MachOFatFile64 {
    () => {
        deps!();
        # [doc = " A 64-bit Mach-O universal binary."] # [doc = ""] # [doc = " This is a file that starts with [`macho::FatHeader`], and corresponds"] # [doc = " to [`crate::FileKind::MachOFat64`]."] pub type MachOFatFile64 < 'data > = MachOFatFile < 'data , macho :: FatArch64 > ;
    };
}

MachOFatFile64!()