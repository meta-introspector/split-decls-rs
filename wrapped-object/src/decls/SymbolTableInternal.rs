macro_rules! deps {
    () => {
        WasmSymbolTable!();
        Endianness!();
        CoffBigSymbolTable!();
        XcoffSymbolTable64!();
        ReadRef!();
        ElfSymbolTable64!();
        CoffSymbolTable!();
        MachO32!();
        MachO64!();
        MachOSymbolTable32!();
        MachOSymbolTable64!();
        XcoffSymbolTable32!();
        ElfSymbolTable32!();
    };
}

macro_rules! SymbolTableInternal {
    () => {
        deps!();
        # [derive (Debug)] enum SymbolTableInternal < 'data , 'file , R > where R : ReadRef < 'data > , { # [cfg (feature = "coff")] Coff ((coff :: CoffSymbolTable < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "coff")] CoffBig ((coff :: CoffBigSymbolTable < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "elf")] Elf32 ((elf :: ElfSymbolTable32 < 'data , 'file , Endianness , R > , PhantomData < R > ,) ,) , # [cfg (feature = "elf")] Elf64 ((elf :: ElfSymbolTable64 < 'data , 'file , Endianness , R > , PhantomData < R > ,) ,) , # [cfg (feature = "macho")] MachO32 ((macho :: MachOSymbolTable32 < 'data , 'file , Endianness , R > , PhantomData < () > ,) ,) , # [cfg (feature = "macho")] MachO64 ((macho :: MachOSymbolTable64 < 'data , 'file , Endianness , R > , PhantomData < () > ,) ,) , # [cfg (feature = "pe")] Pe32 ((coff :: CoffSymbolTable < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "pe")] Pe64 ((coff :: CoffSymbolTable < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "wasm")] Wasm ((wasm :: WasmSymbolTable < 'data , 'file > , PhantomData < R >)) , # [cfg (feature = "xcoff")] Xcoff32 ((xcoff :: XcoffSymbolTable32 < 'data , 'file , R > , PhantomData < R >)) , # [cfg (feature = "xcoff")] Xcoff64 ((xcoff :: XcoffSymbolTable64 < 'data , 'file , R > , PhantomData < R >)) , }
    };
}

SymbolTableInternal!()