macro_rules! deps {
    () => {
        FatHeader!();
        FatArch!();
    };
}

macro_rules! MachOFatFile {
    () => {
        deps!();
        # [doc = " A Mach-O universal binary."] # [doc = ""] # [doc = " This is a file that starts with [`macho::FatHeader`], and corresponds"] # [doc = " to [`crate::FileKind::MachOFat32`] or [`crate::FileKind::MachOFat64`]."] # [derive (Debug , Clone)] pub struct MachOFatFile < 'data , Fat : FatArch > { header : & 'data macho :: FatHeader , arches : & 'data [Fat] , }
    };
}

MachOFatFile!()