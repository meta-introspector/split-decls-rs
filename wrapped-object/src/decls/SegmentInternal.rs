macro_rules! deps {
    () => {
        MachO64!();
        ElfSegment64!();
        PeSegment32!();
        MachOSegment32!();
        PeSegment64!();
        CoffSegment!();
        XcoffSegment32!();
        ElfSegment32!();
        MachO32!();
        CoffBigSegment!();
        MachOSegment64!();
        WasmSegment!();
        XcoffSegment64!();
        ReadRef!();
        Endianness!();
    };
}

macro_rules! SegmentInternal {
    () => {
        deps!();
        # [derive (Debug)] enum SegmentInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffSegment < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigSegment < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfSegment32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfSegment64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOSegment32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOSegment64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeSegment32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeSegment64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmSegment < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffSegment32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffSegment64 < 'data , 'file , R >) , }
    };
}

SegmentInternal!();