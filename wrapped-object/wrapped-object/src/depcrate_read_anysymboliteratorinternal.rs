// Generated macro for SymbolIteratorInternal (enum)
macro_rules! Depcrate_read_anySymbolIteratorInternal {
() => {
// Module: crate::read::any
// Provides: {"SymbolIteratorInternal"}
// Dependencies: {}
# [derive (Debug)] enum SymbolIteratorInternal < 'data , 'file , R > where R : ReadRef < 'data > , { # [cfg (feature = "coff")] Coff ((coff :: CoffSymbolIterator < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "coff")] CoffBig ((coff :: CoffBigSymbolIterator < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "elf")] Elf32 ((elf :: ElfSymbolIterator32 < 'data , 'file , Endianness , R > , PhantomData < R > ,) ,) , # [cfg (feature = "elf")] Elf64 ((elf :: ElfSymbolIterator64 < 'data , 'file , Endianness , R > , PhantomData < R > ,) ,) , # [cfg (feature = "macho")] MachO32 ((macho :: MachOSymbolIterator32 < 'data , 'file , Endianness , R > , PhantomData < () > ,) ,) , # [cfg (feature = "macho")] MachO64 ((macho :: MachOSymbolIterator64 < 'data , 'file , Endianness , R > , PhantomData < () > ,) ,) , # [cfg (feature = "pe")] Pe32 ((coff :: CoffSymbolIterator < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "pe")] Pe64 ((coff :: CoffSymbolIterator < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "wasm")] Wasm ((wasm :: WasmSymbolIterator < 'data , 'file > , PhantomData < R >)) , # [cfg (feature = "xcoff")] Xcoff32 ((xcoff :: XcoffSymbolIterator32 < 'data , 'file , R > , PhantomData < R > ,) ,) , # [cfg (feature = "xcoff")] Xcoff64 ((xcoff :: XcoffSymbolIterator64 < 'data , 'file , R > , PhantomData < R > ,) ,) , }
};
}
