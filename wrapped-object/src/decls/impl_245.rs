macro_rules! deps {
    () => {
        Section!();
        CoffHeader!();
        Result!();
        SymbolScope!();
        ReadRef!();
        ObjectSymbol!();
        CoffSection!();
        SymbolIndex!();
        File!();
        SymbolKind!();
        SectionIndex!();
        CoffSymbol!();
        SymbolFlags!();
        SymbolSection!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > ObjectSymbol < 'data > for CoffSymbol < 'data , 'file , R , Coff > { # [inline] fn index (& self) -> SymbolIndex { self . index } fn name_bytes (& self) -> read :: Result < & 'data [u8] > { if self . symbol . has_aux_file_name () { self . file . symbols . aux_file_name (self . index , self . symbol . number_of_aux_symbols ()) } else { self . symbol . name (self . file . symbols . strings ()) } } fn name (& self) -> read :: Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 COFF symbol name") } fn address (& self) -> u64 { self . symbol . address (self . file . image_base , & self . file . sections) . unwrap_or (None) . unwrap_or (0) } fn size (& self) -> u64 { match self . symbol . storage_class () { pe :: IMAGE_SYM_CLASS_STATIC => { if self . symbol . has_aux_section () { if let Ok (aux) = self . file . symbols . aux_section (self . index) { u64 :: from (aux . length . get (LE)) } else { 0 } } else { 0 } } pe :: IMAGE_SYM_CLASS_EXTERNAL => { if self . symbol . section_number () == pe :: IMAGE_SYM_UNDEFINED { u64 :: from (self . symbol . value ()) } else if self . symbol . has_aux_function () { if let Ok (aux) = self . file . symbols . aux_function (self . index) { u64 :: from (aux . total_size . get (LE)) } else { 0 } } else { 0 } } _ => 0 , } } fn kind (& self) -> SymbolKind { let derived_kind = if self . symbol . derived_type () == pe :: IMAGE_SYM_DTYPE_FUNCTION { SymbolKind :: Text } else { SymbolKind :: Data } ; match self . symbol . storage_class () { pe :: IMAGE_SYM_CLASS_STATIC => { if self . symbol . has_aux_section () { SymbolKind :: Section } else { derived_kind } } pe :: IMAGE_SYM_CLASS_EXTERNAL | pe :: IMAGE_SYM_CLASS_WEAK_EXTERNAL => derived_kind , pe :: IMAGE_SYM_CLASS_SECTION => SymbolKind :: Section , pe :: IMAGE_SYM_CLASS_FILE => SymbolKind :: File , pe :: IMAGE_SYM_CLASS_LABEL => SymbolKind :: Label , _ => SymbolKind :: Unknown , } } fn section (& self) -> SymbolSection { match self . symbol . section_number () { pe :: IMAGE_SYM_UNDEFINED => { if self . symbol . storage_class () == pe :: IMAGE_SYM_CLASS_EXTERNAL { if self . symbol . value () == 0 { SymbolSection :: Undefined } else { SymbolSection :: Common } } else if self . symbol . storage_class () == pe :: IMAGE_SYM_CLASS_SECTION { SymbolSection :: Undefined } else { SymbolSection :: Unknown } } pe :: IMAGE_SYM_ABSOLUTE => SymbolSection :: Absolute , pe :: IMAGE_SYM_DEBUG => { if self . symbol . storage_class () == pe :: IMAGE_SYM_CLASS_FILE { SymbolSection :: None } else { SymbolSection :: Unknown } } index if index > 0 => SymbolSection :: Section (SectionIndex (index as usize)) , _ => SymbolSection :: Unknown , } } # [inline] fn is_undefined (& self) -> bool { self . symbol . storage_class () == pe :: IMAGE_SYM_CLASS_EXTERNAL && self . symbol . section_number () == pe :: IMAGE_SYM_UNDEFINED && self . symbol . value () == 0 } # [inline] fn is_definition (& self) -> bool { self . symbol . is_definition () } # [inline] fn is_common (& self) -> bool { self . symbol . storage_class () == pe :: IMAGE_SYM_CLASS_EXTERNAL && self . symbol . section_number () == pe :: IMAGE_SYM_UNDEFINED && self . symbol . value () != 0 } # [inline] fn is_weak (& self) -> bool { self . symbol . storage_class () == pe :: IMAGE_SYM_CLASS_WEAK_EXTERNAL } # [inline] fn scope (& self) -> SymbolScope { match self . symbol . storage_class () { pe :: IMAGE_SYM_CLASS_EXTERNAL | pe :: IMAGE_SYM_CLASS_WEAK_EXTERNAL => { SymbolScope :: Linkage } _ => SymbolScope :: Compilation , } } # [inline] fn is_global (& self) -> bool { match self . symbol . storage_class () { pe :: IMAGE_SYM_CLASS_EXTERNAL | pe :: IMAGE_SYM_CLASS_WEAK_EXTERNAL => true , _ => false , } } # [inline] fn is_local (& self) -> bool { ! self . is_global () } fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > { if self . symbol . has_aux_section () { if let Ok (aux) = self . file . symbols . aux_section (self . index) { let number = if Coff :: is_type_bigobj () { u32 :: from (aux . number . get (LE)) | (u32 :: from (aux . high_number . get (LE)) << 16) } else { u32 :: from (aux . number . get (LE)) } ; return SymbolFlags :: CoffSection { selection : aux . selection , associative_section : if number == 0 { None } else { Some (SectionIndex (number as usize)) } , } ; } } SymbolFlags :: None } }
    };
}

impl_245!();