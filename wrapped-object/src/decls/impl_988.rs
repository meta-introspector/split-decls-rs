macro_rules! deps {
    () => {
        MachOBuildVersion!();
        Endianness!();
        BuildVersionCommand!();
    };
}

macro_rules! impl_988 {
    () => {
        deps!();
        impl MachOBuildVersion { fn cmdsize (& self) -> u32 { let sz = mem :: size_of :: < macho :: BuildVersionCommand < Endianness > > () ; debug_assert ! (sz <= u32 :: MAX as usize) ; sz as u32 } }
    };
}

impl_988!();