macro_rules! deps {
    () => {
        FatArch32!();
        FatArch!();
        BigEndian!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl FatArch for FatArch32 { type Word = u32 ; const MAGIC : u32 = macho :: FAT_MAGIC ; fn cputype (& self) -> u32 { self . cputype . get (BigEndian) } fn cpusubtype (& self) -> u32 { self . cpusubtype . get (BigEndian) } fn offset (& self) -> Self :: Word { self . offset . get (BigEndian) } fn size (& self) -> Self :: Word { self . size . get (BigEndian) } fn align (& self) -> u32 { self . align . get (BigEndian) } }
    };
}

impl_524!()