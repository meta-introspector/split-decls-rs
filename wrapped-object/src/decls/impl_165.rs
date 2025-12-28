macro_rules! deps {
    () => {
        SymbolIndex!();
        Result!();
        SymbolFlags!();
        SymbolKind!();
        SymbolSection!();
        Symbol!();
        ReadRef!();
        SymbolInternal!();
        SymbolScope!();
        SectionIndex!();
        ObjectSymbol!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > ObjectSymbol < 'data > for Symbol < 'data , 'file , R > { fn index (& self) -> SymbolIndex { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . index ()) } fn name_bytes (& self) -> Result < & 'data [u8] > { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . name_bytes ()) } fn name (& self) -> Result < & 'data str > { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . name ()) } fn address (& self) -> u64 { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . address ()) } fn size (& self) -> u64 { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . size ()) } fn kind (& self) -> SymbolKind { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . kind ()) } fn section (& self) -> SymbolSection { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . section ()) } fn is_undefined (& self) -> bool { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . is_undefined ()) } fn is_definition (& self) -> bool { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . is_definition ()) } fn is_common (& self) -> bool { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . is_common ()) } fn is_weak (& self) -> bool { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . is_weak ()) } fn scope (& self) -> SymbolScope { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . scope ()) } fn is_global (& self) -> bool { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . is_global ()) } fn is_local (& self) -> bool { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . is_local ()) } fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > { with_inner ! (self . inner , SymbolInternal , | x | x . 0 . flags ()) } }
    };
}

impl_165!();