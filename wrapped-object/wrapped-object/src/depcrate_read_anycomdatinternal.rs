// Generated macro for ComdatInternal (enum)
macro_rules! Depcrate_read_anyComdatInternal {
() => {
// Module: crate::read::any
// Provides: {"ComdatInternal"}
// Dependencies: {}
enum ComdatInternal < 'data , 'file , R : ReadRef < 'data > > { # [cfg (feature = "coff")] Coff (coff :: CoffComdat < 'data , 'file , R >) , # [cfg (feature = "coff")] CoffBig (coff :: CoffBigComdat < 'data , 'file , R >) , # [cfg (feature = "elf")] Elf32 (elf :: ElfComdat32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfComdat64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO32 (macho :: MachOComdat32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "macho")] MachO64 (macho :: MachOComdat64 < 'data , 'file , Endianness , R >) , # [cfg (feature = "pe")] Pe32 (pe :: PeComdat32 < 'data , 'file , R >) , # [cfg (feature = "pe")] Pe64 (pe :: PeComdat64 < 'data , 'file , R >) , # [cfg (feature = "wasm")] Wasm (wasm :: WasmComdat < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff32 (xcoff :: XcoffComdat32 < 'data , 'file , R >) , # [cfg (feature = "xcoff")] Xcoff64 (xcoff :: XcoffComdat64 < 'data , 'file , R >) , }
};
}
