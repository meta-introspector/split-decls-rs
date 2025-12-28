macro_rules! deps {
    () => {
        Note!();
        SymbolScope!();
        Symbol!();
        SymbolFlags!();
        Object!();
        SymbolKind!();
        SymbolSection!();
        SectionIndex!();
        Result!();
        SymbolIndex!();
    };
}

macro_rules! ObjectSymbol {
    () => {
        deps!();
        # [doc = " A symbol table entry in an [`Object`]."] # [doc = ""] # [doc = " This trait is part of the unified read API."] pub trait ObjectSymbol < 'data > : read :: private :: Sealed { # [doc = " The index of the symbol."] fn index (& self) -> SymbolIndex ; # [doc = " The name of the symbol."] fn name_bytes (& self) -> Result < & 'data [u8] > ; # [doc = " The name of the symbol."] # [doc = ""] # [doc = " Returns an error if the name is not UTF-8."] fn name (& self) -> Result < & 'data str > ; # [doc = " The address of the symbol. May be zero if the address is unknown."] fn address (& self) -> u64 ; # [doc = " The size of the symbol. May be zero if the size is unknown."] fn size (& self) -> u64 ; # [doc = " Return the kind of this symbol."] fn kind (& self) -> SymbolKind ; # [doc = " Returns the section where the symbol is defined."] fn section (& self) -> SymbolSection ; # [doc = " Returns the section index for the section containing this symbol."] # [doc = ""] # [doc = " May return `None` if the symbol is not defined in a section."] fn section_index (& self) -> Option < SectionIndex > { self . section () . index () } # [doc = " Return true if the symbol is undefined."] fn is_undefined (& self) -> bool ; # [doc = " Return true if the symbol is a definition of a function or data object"] # [doc = " that has a known address."] # [doc = ""] # [doc = " This is primarily used to implement [`Object::symbol_map`]."] fn is_definition (& self) -> bool ; # [doc = " Return true if the symbol is common data."] # [doc = ""] # [doc = " Note: does not check for [`SymbolSection::Section`] with [`SectionKind::Common`]."] fn is_common (& self) -> bool ; # [doc = " Return true if the symbol is weak."] fn is_weak (& self) -> bool ; # [doc = " Returns the symbol scope."] fn scope (& self) -> SymbolScope ; # [doc = " Return true if the symbol visible outside of the compilation unit."] # [doc = ""] # [doc = " This treats [`SymbolScope::Unknown`] as global."] fn is_global (& self) -> bool ; # [doc = " Return true if the symbol is only visible within the compilation unit."] fn is_local (& self) -> bool ; # [doc = " Symbol flags that are specific to each file format."] fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > ; }
    };
}

ObjectSymbol!();