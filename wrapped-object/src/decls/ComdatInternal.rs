macro_rules! deps {
    () => {
        MachOComdat32!();
        ReadRef!();
        ElfComdat32!();
        MachO64!();
        XcoffComdat32!();
        ElfComdat64!();
        MachOComdat64!();
        MachO32!();
        PeComdat32!();
        CoffBigComdat!();
        PeComdat64!();
        CoffComdat!();
        WasmComdat!();
        XcoffComdat64!();
        Endianness!();
    };
}

macro_rules! ComdatInternal {
    () => {
        deps!();
        enum ComdatInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffComdat < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigComdat < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfComdat32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfComdat64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOComdat32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOComdat64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeComdat32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeComdat64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmComdat < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffComdat32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffComdat64 < 'data , 'file , R >) , }
    };
}

ComdatInternal!();