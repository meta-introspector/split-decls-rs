macro_rules! deps {
    () => {
        CoffHeader!();
        ReadRef!();
        CoffComdat!();
        ObjectComdat!();
        ComdatKind!();
        SectionIterator!();
        SymbolIndex!();
        Result!();
        CoffComdatSectionIterator!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > ObjectComdat < 'data > for CoffComdat < 'data , 'file , R , Coff > { type SectionIterator = CoffComdatSectionIterator < 'data , 'file , R , Coff > ; # [inline] fn kind (& self) -> ComdatKind { match self . selection { pe :: IMAGE_COMDAT_SELECT_NODUPLICATES => ComdatKind :: NoDuplicates , pe :: IMAGE_COMDAT_SELECT_ANY => ComdatKind :: Any , pe :: IMAGE_COMDAT_SELECT_SAME_SIZE => ComdatKind :: SameSize , pe :: IMAGE_COMDAT_SELECT_EXACT_MATCH => ComdatKind :: ExactMatch , pe :: IMAGE_COMDAT_SELECT_LARGEST => ComdatKind :: Largest , pe :: IMAGE_COMDAT_SELECT_NEWEST => ComdatKind :: Newest , _ => ComdatKind :: Unknown , } } # [inline] fn symbol (& self) -> SymbolIndex { self . symbol_index } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { self . symbol . name (self . file . common . symbols . strings ()) } # [inline] fn name (& self) -> Result < & 'data str > { let bytes = self . name_bytes () ? ; str :: from_utf8 (bytes) . ok () . read_error ("Non UTF-8 COFF COMDAT name") } # [inline] fn sections (& self) -> Self :: SectionIterator { CoffComdatSectionIterator { file : self . file , section_number : self . symbol . section_number () , index : SymbolIndex (0) , } } }
    };
}

impl_265!();