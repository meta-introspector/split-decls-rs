macro_rules! deps {
    () => {
        WasmFile!();
        U64!();
        MachOFile32!();
        ReadRef!();
        File!();
        Error!();
        PeFile64!();
        MachOFile64!();
        ElfFile64!();
        CoffBigFile!();
        MachO64!();
        XcoffFile32!();
        MachO!();
        MachO32!();
        XcoffFile64!();
        U32!();
        DyldCacheImage!();
        Endian!();
        PeFile32!();
        CoffFile!();
        Result!();
        BinaryFormat!();
        FileKind!();
        ElfFile32!();
        AddressSize!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > > File < 'data , R > { # [doc = " Parse the raw file data."] pub fn parse (data : R) -> Result < Self > { Ok (match FileKind :: parse (data) ? { # [cfg (feature = "elf")] FileKind :: Elf32 => File :: Elf32 (elf :: ElfFile32 :: parse (data) ?) , # [cfg (feature = "elf")] FileKind :: Elf64 => File :: Elf64 (elf :: ElfFile64 :: parse (data) ?) , # [cfg (feature = "macho")] FileKind :: MachO32 => File :: MachO32 (macho :: MachOFile32 :: parse (data) ?) , # [cfg (feature = "macho")] FileKind :: MachO64 => File :: MachO64 (macho :: MachOFile64 :: parse (data) ?) , # [cfg (feature = "wasm")] FileKind :: Wasm => File :: Wasm (wasm :: WasmFile :: parse (data) ?) , # [cfg (feature = "pe")] FileKind :: Pe32 => File :: Pe32 (pe :: PeFile32 :: parse (data) ?) , # [cfg (feature = "pe")] FileKind :: Pe64 => File :: Pe64 (pe :: PeFile64 :: parse (data) ?) , # [cfg (feature = "coff")] FileKind :: Coff => File :: Coff (coff :: CoffFile :: parse (data) ?) , # [cfg (feature = "coff")] FileKind :: CoffBig => File :: CoffBig (coff :: CoffBigFile :: parse (data) ?) , # [cfg (feature = "xcoff")] FileKind :: Xcoff32 => File :: Xcoff32 (xcoff :: XcoffFile32 :: parse (data) ?) , # [cfg (feature = "xcoff")] FileKind :: Xcoff64 => File :: Xcoff64 (xcoff :: XcoffFile64 :: parse (data) ?) , # [allow (unreachable_patterns)] _ => return Err (Error ("Unsupported file format")) , }) } # [doc = " Parse a Mach-O image from the dyld shared cache."] # [cfg (feature = "macho")] pub fn parse_dyld_cache_image < 'cache , E : crate :: Endian > (image : & macho :: DyldCacheImage < 'data , 'cache , E , R > ,) -> Result < Self > { Ok (match image . cache . architecture () . address_size () { Some (read :: AddressSize :: U64) => { File :: MachO64 (macho :: MachOFile64 :: parse_dyld_cache_image (image) ?) } Some (read :: AddressSize :: U32) => { File :: MachO32 (macho :: MachOFile32 :: parse_dyld_cache_image (image) ?) } _ => return Err (Error ("Unsupported file format")) , }) } # [doc = " Return the file format."] pub fn format (& self) -> BinaryFormat { match self { # [cfg (feature = "coff")] File :: Coff (_) | File :: CoffBig (_) => BinaryFormat :: Coff , # [cfg (feature = "elf")] File :: Elf32 (_) | File :: Elf64 (_) => BinaryFormat :: Elf , # [cfg (feature = "macho")] File :: MachO32 (_) | File :: MachO64 (_) => BinaryFormat :: MachO , # [cfg (feature = "pe")] File :: Pe32 (_) | File :: Pe64 (_) => BinaryFormat :: Pe , # [cfg (feature = "wasm")] File :: Wasm (_) => BinaryFormat :: Wasm , # [cfg (feature = "xcoff")] File :: Xcoff32 (_) | File :: Xcoff64 (_) => BinaryFormat :: Xcoff , } } }
    };
}

impl_124!();