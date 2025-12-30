// Generated macro for SegmentInternal (enum)
macro_rules! Depcrate_read_anySegmentInternal {
() => {
// Module: crate::read::any
// Provides: {"SegmentInternal"}
// Dependencies: {}
# [derive (Debug)] enum SegmentInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffSegment < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigSegment < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfSegment32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfSegment64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOSegment32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOSegment64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeSegment32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeSegment64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmSegment < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffSegment32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffSegment64 < 'data , 'file , R >) , }
};
}
