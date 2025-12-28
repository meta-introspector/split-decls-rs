macro_rules! deps {
    () => {
        ObjectKind!();
        Export!();
        DynamicRelocationIterator!();
        Architecture!();
        SectionIndex!();
        CoffComdatIterator!();
        CoffSymbolIterator!();
        SectionIterator!();
        CoffComdat!();
        CoffSymbol!();
        SymbolTable!();
        SegmentIterator!();
        CoffFile!();
        CoffSegment!();
        CoffSectionIterator!();
        Object!();
        SymbolIterator!();
        NoDynamicRelocationIterator!();
        CoffSection!();
        Symbol!();
        Segment!();
        FileFlags!();
        CoffSegmentIterator!();
        Section!();
        CoffHeader!();
        SymbolIndex!();
        Import!();
        SubArchitecture!();
        ReadRef!();
        Result!();
        CoffSymbolTable!();
        ComdatIterator!();
        Comdat!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < 'data , R , Coff > Object < 'data > for CoffFile < 'data , R , Coff > where R : ReadRef < 'data > , Coff : CoffHeader , { type Segment < 'file > = CoffSegment < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type SegmentIterator < 'file > = CoffSegmentIterator < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type Section < 'file > = CoffSection < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type SectionIterator < 'file > = CoffSectionIterator < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type Comdat < 'file > = CoffComdat < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type ComdatIterator < 'file > = CoffComdatIterator < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type Symbol < 'file > = CoffSymbol < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type SymbolIterator < 'file > = CoffSymbolIterator < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type SymbolTable < 'file > = CoffSymbolTable < 'data , 'file , R , Coff > where Self : 'file , 'data : 'file ; type DynamicRelocationIterator < 'file > = NoDynamicRelocationIterator where Self : 'file , 'data : 'file ; fn architecture (& self) -> Architecture { match self . header . machine () { pe :: IMAGE_FILE_MACHINE_ARMNT => Architecture :: Arm , pe :: IMAGE_FILE_MACHINE_ARM64 | pe :: IMAGE_FILE_MACHINE_ARM64EC => Architecture :: Aarch64 , pe :: IMAGE_FILE_MACHINE_I386 => Architecture :: I386 , pe :: IMAGE_FILE_MACHINE_AMD64 => Architecture :: X86_64 , pe :: IMAGE_FILE_MACHINE_POWERPC | pe :: IMAGE_FILE_MACHINE_POWERPCFP | pe :: IMAGE_FILE_MACHINE_POWERPCBE => Architecture :: PowerPc , _ => Architecture :: Unknown , } } fn sub_architecture (& self) -> Option < SubArchitecture > { match self . header . machine () { pe :: IMAGE_FILE_MACHINE_ARM64EC => Some (SubArchitecture :: Arm64EC) , _ => None , } } # [inline] fn is_little_endian (& self) -> bool { match self . header . machine () { pe :: IMAGE_FILE_MACHINE_POWERPCBE => false , _ => true , } } # [inline] fn is_64 (& self) -> bool { false } fn kind (& self) -> ObjectKind { ObjectKind :: Relocatable } fn segments (& self) -> CoffSegmentIterator < 'data , '_ , R , Coff > { CoffSegmentIterator { file : self , iter : self . common . sections . iter () , } } fn section_by_name_bytes < 'file > (& 'file self , section_name : & [u8] ,) -> Option < CoffSection < 'data , 'file , R , Coff > > { self . sections () . find (| section | section . name_bytes () == Ok (section_name)) } fn section_by_index (& self , index : SectionIndex) -> Result < CoffSection < 'data , '_ , R , Coff > > { let section = self . common . sections . section (index) ? ; Ok (CoffSection { file : self , index , section , }) } fn sections (& self) -> CoffSectionIterator < 'data , '_ , R , Coff > { CoffSectionIterator { file : self , iter : self . common . sections . iter () . enumerate () , } } fn comdats (& self) -> CoffComdatIterator < 'data , '_ , R , Coff > { CoffComdatIterator :: new (self) } fn symbol_by_index (& self , index : SymbolIndex) -> Result < CoffSymbol < 'data , '_ , R , Coff > > { let symbol = self . common . symbols . symbol (index) ? ; Ok (CoffSymbol { file : & self . common , index , symbol , }) } fn symbols (& self) -> CoffSymbolIterator < 'data , '_ , R , Coff > { CoffSymbolIterator :: new (& self . common) } # [inline] fn symbol_table (& self) -> Option < CoffSymbolTable < 'data , '_ , R , Coff > > { Some (CoffSymbolTable { file : & self . common }) } fn dynamic_symbols (& self) -> CoffSymbolIterator < 'data , '_ , R , Coff > { CoffSymbolIterator :: empty (& self . common) } # [inline] fn dynamic_symbol_table (& self) -> Option < CoffSymbolTable < 'data , '_ , R , Coff > > { None } # [inline] fn dynamic_relocations (& self) -> Option < NoDynamicRelocationIterator > { None } # [inline] fn imports (& self) -> Result < Vec < Import < 'data > > > { Ok (Vec :: new ()) } # [inline] fn exports (& self) -> Result < Vec < Export < 'data > > > { Ok (Vec :: new ()) } fn has_debug_symbols (& self) -> bool { self . section_by_name (".debug_info") . is_some () } fn relative_address_base (& self) -> u64 { 0 } # [inline] fn entry (& self) -> u64 { 0 } fn flags (& self) -> FileFlags { FileFlags :: Coff { characteristics : self . header . characteristics () , } } }
    };
}

impl_199!();