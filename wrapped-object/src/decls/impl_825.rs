macro_rules! deps {
    () => {
        XcoffSymbol!();
        SectionIndex!();
        ObjectSymbol!();
        SymbolIndex!();
        SymbolFlags!();
        File!();
        SymbolKind!();
        Section!();
        Result!();
        FileHeader!();
        Dynamic!();
        SymbolScope!();
        ReadRef!();
        SymbolSection!();
    };
}

macro_rules! impl_825 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > ObjectSymbol < 'data > for XcoffSymbol < 'data , 'file , Xcoff , R > { # [inline] fn index (& self) -> SymbolIndex { self . index } fn name_bytes (& self) -> Result < & 'data [u8] > { if self . symbol . has_aux_file () { self . symbols . aux_file (self . index , 1) ? . fname (self . symbols . strings) } else { self . symbol . name (self . symbols . strings) } } fn name (& self) -> Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 XCOFF symbol name") } # [inline] fn address (& self) -> u64 { match self . symbol . n_sclass () { xcoff :: C_EXT | xcoff :: C_WEAKEXT | xcoff :: C_HIDEXT | xcoff :: C_FCN | xcoff :: C_BLOCK | xcoff :: C_STAT | xcoff :: C_INFO => self . symbol . n_value () . into () , _ => 0 , } } # [inline] fn size (& self) -> u64 { if self . symbol . has_aux_csect () { if let Ok (aux_csect) = self . file . symbols . aux_csect (self . index , self . symbol . n_numaux () as usize) { let sym_type = aux_csect . sym_type () ; if sym_type == xcoff :: XTY_SD || sym_type == xcoff :: XTY_CM { return aux_csect . x_scnlen () ; } } } 0 } fn kind (& self) -> SymbolKind { if self . symbol . has_aux_csect () { if let Ok (aux_csect) = self . file . symbols . aux_csect (self . index , self . symbol . n_numaux () as usize) { let sym_type = aux_csect . sym_type () ; if sym_type == xcoff :: XTY_SD || sym_type == xcoff :: XTY_CM { return match aux_csect . x_smclas () { xcoff :: XMC_PR | xcoff :: XMC_GL => SymbolKind :: Text , xcoff :: XMC_RO | xcoff :: XMC_RW | xcoff :: XMC_TD | xcoff :: XMC_BS => { SymbolKind :: Data } xcoff :: XMC_TL | xcoff :: XMC_UL => SymbolKind :: Tls , xcoff :: XMC_DS | xcoff :: XMC_TC0 | xcoff :: XMC_TC => { SymbolKind :: Data } _ => SymbolKind :: Unknown , } ; } else if sym_type == xcoff :: XTY_LD { return SymbolKind :: Text ; } else if sym_type == xcoff :: XTY_ER { return SymbolKind :: Unknown ; } } } match self . symbol . n_sclass () { xcoff :: C_FILE => SymbolKind :: File , _ => SymbolKind :: Unknown , } } fn section (& self) -> SymbolSection { match self . symbol . n_scnum () { xcoff :: N_ABS => SymbolSection :: Absolute , xcoff :: N_UNDEF => SymbolSection :: Undefined , xcoff :: N_DEBUG => SymbolSection :: None , index if index > 0 => SymbolSection :: Section (SectionIndex (index as usize)) , _ => SymbolSection :: Unknown , } } # [inline] fn is_undefined (& self) -> bool { self . symbol . is_undefined () } # [doc = " Return true if the symbol is a definition of a function or data object."] # [inline] fn is_definition (& self) -> bool { if self . symbol . n_scnum () <= 0 { return false ; } if self . symbol . has_aux_csect () { if let Ok (aux_csect) = self . symbols . aux_csect (self . index , self . symbol . n_numaux () as usize) { let sym_type = aux_csect . sym_type () ; sym_type == xcoff :: XTY_SD || sym_type == xcoff :: XTY_LD || sym_type == xcoff :: XTY_CM } else { false } } else { false } } # [inline] fn is_common (& self) -> bool { self . symbol . n_sclass () == xcoff :: C_EXT && self . symbol . n_scnum () == xcoff :: N_UNDEF } # [inline] fn is_weak (& self) -> bool { self . symbol . n_sclass () == xcoff :: C_WEAKEXT } fn scope (& self) -> SymbolScope { if self . symbol . n_scnum () == xcoff :: N_UNDEF { SymbolScope :: Unknown } else { match self . symbol . n_sclass () { xcoff :: C_EXT | xcoff :: C_WEAKEXT => { let visibility = self . symbol . n_type () & xcoff :: SYM_V_MASK ; if visibility == xcoff :: SYM_V_HIDDEN { SymbolScope :: Linkage } else { SymbolScope :: Dynamic } } _ => SymbolScope :: Compilation , } } } # [inline] fn is_global (& self) -> bool { match self . symbol . n_sclass () { xcoff :: C_EXT | xcoff :: C_WEAKEXT => true , _ => false , } } # [inline] fn is_local (& self) -> bool { ! self . is_global () } # [inline] fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > { let mut x_smtyp = 0 ; let mut x_smclas = 0 ; let mut containing_csect = None ; if self . symbol . has_aux_csect () { if let Ok (aux_csect) = self . file . symbols . aux_csect (self . index , self . symbol . n_numaux () as usize) { x_smtyp = aux_csect . x_smtyp () ; x_smclas = aux_csect . x_smclas () ; if aux_csect . sym_type () == xcoff :: XTY_LD { containing_csect = Some (SymbolIndex (aux_csect . x_scnlen () as usize)) } } } SymbolFlags :: Xcoff { n_sclass : self . symbol . n_sclass () , x_smtyp , x_smclas , containing_csect , } } }
    };
}

impl_825!();