macro_rules! deps {
    () => {
        IMAGE_SECTION_HEADER!();
    };
}

macro_rules! offset_from_rva {
    () => {
        deps!();
        fn offset_from_rva (section : & IMAGE_SECTION_HEADER , rva : u32) -> usize { (rva - section . VirtualAddress + section . PointerToRawData) as usize }
    };
}

offset_from_rva!();