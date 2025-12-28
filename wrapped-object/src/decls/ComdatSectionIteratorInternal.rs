macro_rules! deps {
    () => {
        MachOComdatSectionIterator64!();
        PeComdatSectionIterator64!();
        ElfComdatSectionIterator32!();
        XcoffComdatSectionIterator64!();
        WasmComdatSectionIterator!();
        Endianness!();
        CoffComdatSectionIterator!();
        ElfComdatSectionIterator64!();
        ReadRef!();
        PeComdatSectionIterator32!();
        XcoffComdatSectionIterator32!();
        MachOComdatSectionIterator32!();
        CoffBigComdatSectionIterator!();
        MachO32!();
        MachO64!();
    };
}

macro_rules! ComdatSectionIteratorInternal {
    () => {
        deps!();
        # [derive (Debug)] enum ComdatSectionIteratorInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffComdatSectionIterator < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigComdatSectionIterator < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfComdatSectionIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfComdatSectionIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOComdatSectionIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOComdatSectionIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeComdatSectionIterator32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeComdatSectionIterator64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmComdatSectionIterator < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffComdatSectionIterator32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffComdatSectionIterator64 < 'data , 'file , R >) , }
    };
}

ComdatSectionIteratorInternal!()