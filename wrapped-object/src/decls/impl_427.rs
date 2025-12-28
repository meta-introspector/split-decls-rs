macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < 'data > Version < 'data > { # [doc = " Return the version name."] pub fn name (& self) -> & 'data [u8] { self . name } # [doc = " Return hash of the version name."] pub fn hash (& self) -> u32 { self . hash } # [doc = " Return the filename of the library containing this version."] # [doc = ""] # [doc = " This is the `vn_file` field of the associated entry in [`elf::SHT_GNU_VERNEED`]."] # [doc = " or `None` if the version info was parsed from a [`elf::SHT_GNU_VERDEF`] section."] pub fn file (& self) -> Option < & 'data [u8] > { self . file } }
    };
}

impl_427!()