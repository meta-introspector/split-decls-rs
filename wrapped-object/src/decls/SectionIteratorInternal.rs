macro_rules! deps {
    () => {
        XcoffSectionIterator64!();
        CoffBigSectionIterator!();
        ElfSectionIterator32!();
        MachO64!();
        CoffSectionIterator!();
        MachO32!();
        ElfSectionIterator64!();
        MachOSectionIterator32!();
        ReadRef!();
        Endianness!();
        PeSectionIterator32!();
        MachOSectionIterator64!();
        PeSectionIterator64!();
        WasmSectionIterator!();
        XcoffSectionIterator32!();
    };
}

macro_rules! SectionIteratorInternal {
    () => {
        deps!();
        # [derive (Debug)] enum SectionIteratorInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffSectionIterator < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigSectionIterator < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfSectionIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfSectionIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOSectionIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOSectionIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeSectionIterator32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeSectionIterator64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmSectionIterator < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffSectionIterator32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffSectionIterator64 < 'data , 'file , R >) , }
    };
}

SectionIteratorInternal!();