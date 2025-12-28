macro_rules! deps {
    () => {
        ReadRef!();
        CoffBigComdatIterator!();
        Endianness!();
        MachOComdatIterator32!();
        ElfComdatIterator32!();
        PeComdatIterator32!();
        XcoffComdatIterator32!();
        XcoffComdatIterator64!();
        MachOComdatIterator64!();
        PeComdatIterator64!();
        ElfComdatIterator64!();
        MachO64!();
        WasmComdatIterator!();
        MachO32!();
        CoffComdatIterator!();
    };
}

macro_rules! ComdatIteratorInternal {
    () => {
        deps!();
        # [derive (Debug)] enum ComdatIteratorInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffComdatIterator < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigComdatIterator < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfComdatIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfComdatIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOComdatIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOComdatIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeComdatIterator32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeComdatIterator64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmComdatIterator < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffComdatIterator32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffComdatIterator64 < 'data , 'file , R >) , }
    };
}

ComdatIteratorInternal!();