macro_rules! deps {
    () => {
        MachOSymbol64!();
        MachO64!();
        XcoffSymbol32!();
        MachOSymbol32!();
        MachO32!();
        CoffBigSymbol!();
        Endianness!();
        ElfSymbol32!();
        XcoffSymbol64!();
        WasmSymbol!();
        ReadRef!();
        ElfSymbol64!();
        CoffSymbol!();
    };
}

macro_rules! SymbolInternal {
    () => {
        deps!();
        enum SymbolInternal < 'data , 'file , R > where R : ReadRef < 'data > , { # [cfg (feature = "coff")] Coff ((coff :: CoffSymbol < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "coff")] CoffBig ((coff :: CoffBigSymbol < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "elf")] Elf32 ((elf :: ElfSymbol32 < 'data , 'file , Endianness , R > , PhantomData < R > ,) ,) , # [cfg (feature = "elf")] Elf64 ((elf :: ElfSymbol64 < 'data , 'file , Endianness , R > , PhantomData < R > ,) ,) , # [cfg (feature = "macho")] MachO32 ((macho :: MachOSymbol32 < 'data , 'file , Endianness , R > , PhantomData < () > ,) ,) , # [cfg (feature = "macho")] MachO64 ((macho :: MachOSymbol64 < 'data , 'file , Endianness , R > , PhantomData < () > ,) ,) , # [cfg (feature = "pe")] Pe32 ((coff :: CoffSymbol < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "pe")] Pe64 ((coff :: CoffSymbol < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "wasm")] Wasm ((wasm :: WasmSymbol < 'data , 'file > , PhantomData < R >)) , # [cfg (feature = "xcoff")] Xcoff32 ((xcoff :: XcoffSymbol32 < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "xcoff")] Xcoff64 ((xcoff :: XcoffSymbol64 < 'data , 'file , R > , PhantomData < R >)) , }
    };
}

SymbolInternal!();