macro_rules! deps {
    () => {
        ElfFile64!();
        PeFile64!();
        PeFile32!();
        XcoffFile64!();
        MachOFile32!();
        Endianness!();
        XcoffFile32!();
        CoffFile!();
        MachO64!();
        ReadRef!();
        MachO32!();
        MachOFile64!();
        WasmFile!();
        Object!();
        CoffBigFile!();
        ElfFile32!();
    };
}

macro_rules! File {
    () => {
        deps!();
        # [doc = " An object file that can be any supported file format."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] # [derive (Debug)] # [non_exhaustive] # [allow (missing_docs)] pub enum File < 'data , R : ReadRef < 'data > = & 'data [u8] > { # [cfg (feature = "coff")] Coff (coff :: CoffFile < 'data , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigFile < 'data , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfFile32 < 'data , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfFile64 < 'data , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOFile32 < 'data , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOFile64 < 'data , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeFile32 < 'data , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeFile64 < 'data , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmFile < 'data , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffFile32 < 'data , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffFile64 < 'data , R >) , }
    };
}

File!()