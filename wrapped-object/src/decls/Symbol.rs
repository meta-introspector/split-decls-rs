macro_rules! deps {
    () => {
        SectionId!();
        ByteString!();
        VersionId!();
        SymbolId!();
        Symbols!();
    };
}

macro_rules! Symbol {
    () => {
        deps!();
        # [doc = " A symbol in [`Symbols`]."] # [doc = ""] # [doc = " This corresponds to [`elf::Sym32`] or [`elf::Sym64`]."] # [derive (Debug)] pub struct Symbol < 'data , const DYNAMIC : bool = false > { id : SymbolId < DYNAMIC > , # [doc = " Ignore this symbol when writing the ELF file."] pub delete : bool , # [doc = " The name of the symbol."] pub name : ByteString < 'data > , # [doc = " The section referenced by the symbol."] # [doc = ""] # [doc = " Used to set the `st_shndx` field in the ELF symbol."] pub section : Option < SectionId > , # [doc = " The `st_info` field in the ELF symbol."] pub st_info : u8 , # [doc = " The `st_other` field in the ELF symbol."] pub st_other : u8 , # [doc = " The `st_shndx` field in the ELF symbol."] # [doc = ""] # [doc = " Only used if `Self::section` is `None`."] pub st_shndx : u16 , # [doc = " The `st_value` field in the ELF symbol."] pub st_value : u64 , # [doc = " The `st_size` field in the ELF symbol."] pub st_size : u64 , # [doc = " GNU version for dynamic symbols."] pub version : VersionId , # [doc = " Set the [`elf::VERSYM_HIDDEN`] flag for this symbol."] pub version_hidden : bool , }
    };
}

Symbol!()