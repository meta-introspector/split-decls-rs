macro_rules! deps {
    () => {
        MachOSegmentIterator32!();
        ReadRef!();
        CoffBigSegmentIterator!();
        XcoffSegmentIterator64!();
        CoffSegmentIterator!();
        PeSegmentIterator64!();
        PeSegmentIterator32!();
        ElfSegmentIterator64!();
        WasmSegmentIterator!();
        MachO32!();
        MachOSegmentIterator64!();
        XcoffSegmentIterator32!();
        MachO64!();
        Endianness!();
        ElfSegmentIterator32!();
    };
}

macro_rules! SegmentIteratorInternal {
    () => {
        deps!();
        # [derive (Debug)] enum SegmentIteratorInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffSegmentIterator < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigSegmentIterator < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfSegmentIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfSegmentIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOSegmentIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOSegmentIterator64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeSegmentIterator32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeSegmentIterator64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmSegmentIterator < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffSegmentIterator32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffSegmentIterator64 < 'data , 'file , R >) , }
    };
}

SegmentIteratorInternal!();