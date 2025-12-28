macro_rules! NonAscii {
    () => {
        pub enum NonAscii { BmpExclAscii (u16) , Astral (char) , }
    };
}

NonAscii!();