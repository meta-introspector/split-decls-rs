macro_rules! deps {
    () => {
        FatArch32!();
        MachOFatFile!();
    };
}

macro_rules! MachOFatFile32 {
    () => {
        deps!();
        # [doc = " A 32-bit Mach-O universal binary."] # [doc = ""] # [doc = " This is a file that starts with [`macho::FatHeader`], and corresponds"] # [doc = " to [`crate::FileKind::MachOFat32`]."] pub type MachOFatFile32 < 'data > = MachOFatFile < 'data , macho :: FatArch32 > ;
    };
}

MachOFatFile32!()