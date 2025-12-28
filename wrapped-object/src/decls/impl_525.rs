macro_rules! deps {
    () => {
        FatArch64!();
        FatArch!();
        BigEndian!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl FatArch for FatArch64 { type Word = u64 ; const MAGIC : u32 = macho :: FAT_MAGIC_64 ; fn cputype (& self) -> u32 { self . cputype . get (BigEndian) } fn cpusubtype (& self) -> u32 { self . cpusubtype . get (BigEndian) } fn offset (& self) -> Self :: Word { self . offset . get (BigEndian) } fn size (& self) -> Self :: Word { self . size . get (BigEndian) } fn align (& self) -> u32 { self . align . get (BigEndian) } }
    };
}

impl_525!()