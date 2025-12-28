macro_rules! deps {
    () => {
        IMAGE_SECTION_HEADER!();
    };
}

macro_rules! section_from_rva {
    () => {
        deps!();
        fn section_from_rva (sections : & [IMAGE_SECTION_HEADER] , rva : u32) -> Option < & IMAGE_SECTION_HEADER > { sections . iter () . find (| & s | { rva >= s . VirtualAddress && rva < s . VirtualAddress + unsafe { s . Misc . VirtualSize } }) }
    };
}

section_from_rva!()