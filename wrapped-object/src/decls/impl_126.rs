macro_rules! deps {
    () => {
        SymbolTableInternal!();
        ObjectMap!();
        DynamicRelocationIteratorInternal!();
        Import!();
        Object!();
        Result!();
        FileFlags!();
        Section!();
        Segment!();
        ObjectKind!();
        SegmentIteratorInternal!();
        SymbolTable!();
        SectionInternal!();
        SymbolIteratorInternal!();
        SymbolMap!();
        CodeView!();
        SectionIteratorInternal!();
        Comdat!();
        DynamicRelocationIterator!();
        SymbolMapName!();
        ComdatIteratorInternal!();
        SymbolIterator!();
        ReadRef!();
        SymbolIndex!();
        SymbolInternal!();
        Export!();
        SectionIndex!();
        File!();
        ComdatIterator!();
        Architecture!();
        Symbol!();
        SegmentIterator!();
        SubArchitecture!();
        SectionIterator!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'data , R > Object < 'data > for File < 'data , R > where R : ReadRef < 'data > , { type Segment < 'file > = Segment < 'data , 'file , R > where Self : 'file , 'data : 'file ; type SegmentIterator < 'file > = SegmentIterator < 'data , 'file , R > where Self : 'file , 'data : 'file ; type Section < 'file > = Section < 'data , 'file , R > where Self : 'file , 'data : 'file ; type SectionIterator < 'file > = SectionIterator < 'data , 'file , R > where Self : 'file , 'data : 'file ; type Comdat < 'file > = Comdat < 'data , 'file , R > where Self : 'file , 'data : 'file ; type ComdatIterator < 'file > = ComdatIterator < 'data , 'file , R > where Self : 'file , 'data : 'file ; type Symbol < 'file > = Symbol < 'data , 'file , R > where Self : 'file , 'data : 'file ; type SymbolIterator < 'file > = SymbolIterator < 'data , 'file , R > where Self : 'file , 'data : 'file ; type SymbolTable < 'file > = SymbolTable < 'data , 'file , R > where Self : 'file , 'data : 'file ; type DynamicRelocationIterator < 'file > = DynamicRelocationIterator < 'data , 'file , R > where Self : 'file , 'data : 'file ; fn architecture (& self) -> Architecture { with_inner ! (self , File , | x | x . architecture ()) } fn sub_architecture (& self) -> Option < SubArchitecture > { with_inner ! (self , File , | x | x . sub_architecture ()) } fn is_little_endian (& self) -> bool { with_inner ! (self , File , | x | x . is_little_endian ()) } fn is_64 (& self) -> bool { with_inner ! (self , File , | x | x . is_64 ()) } fn kind (& self) -> ObjectKind { with_inner ! (self , File , | x | x . kind ()) } fn segments (& self) -> SegmentIterator < 'data , '_ , R > { SegmentIterator { inner : map_inner ! (self , File , SegmentIteratorInternal , | x | x . segments ()) , } } fn section_by_name_bytes < 'file > (& 'file self , section_name : & [u8] ,) -> Option < Section < 'data , 'file , R > > { map_inner_option ! (self , File , SectionInternal , | x | x . section_by_name_bytes (section_name)) . map (| inner | Section { inner }) } fn section_by_index (& self , index : SectionIndex) -> Result < Section < 'data , '_ , R > > { map_inner_option ! (self , File , SectionInternal , | x | x . section_by_index (index)) . map (| inner | Section { inner }) } fn sections (& self) -> SectionIterator < 'data , '_ , R > { SectionIterator { inner : map_inner ! (self , File , SectionIteratorInternal , | x | x . sections ()) , } } fn comdats (& self) -> ComdatIterator < 'data , '_ , R > { ComdatIterator { inner : map_inner ! (self , File , ComdatIteratorInternal , | x | x . comdats ()) , } } fn symbol_by_index (& self , index : SymbolIndex) -> Result < Symbol < 'data , '_ , R > > { map_inner_option ! (self , File , SymbolInternal , | x | x . symbol_by_index (index) . map (| x | (x , PhantomData))) . map (| inner | Symbol { inner }) } fn symbols (& self) -> SymbolIterator < 'data , '_ , R > { SymbolIterator { inner : map_inner ! (self , File , SymbolIteratorInternal , | x | (x . symbols () , PhantomData)) , } } fn symbol_table (& self) -> Option < SymbolTable < 'data , '_ , R > > { map_inner_option ! (self , File , SymbolTableInternal , | x | x . symbol_table () . map (| x | (x , PhantomData))) . map (| inner | SymbolTable { inner }) } fn dynamic_symbols (& self) -> SymbolIterator < 'data , '_ , R > { SymbolIterator { inner : map_inner ! (self , File , SymbolIteratorInternal , | x | (x . dynamic_symbols () , PhantomData)) , } } fn dynamic_symbol_table (& self) -> Option < SymbolTable < 'data , '_ , R > > { map_inner_option ! (self , File , SymbolTableInternal , | x | x . dynamic_symbol_table () . map (| x | (x , PhantomData))) . map (| inner | SymbolTable { inner }) } # [cfg (feature = "elf")] fn dynamic_relocations (& self) -> Option < DynamicRelocationIterator < 'data , '_ , R > > { let inner = match self { File :: Elf32 (ref elf) => { DynamicRelocationIteratorInternal :: Elf32 (elf . dynamic_relocations () ?) } File :: Elf64 (ref elf) => { DynamicRelocationIteratorInternal :: Elf64 (elf . dynamic_relocations () ?) } # [allow (unreachable_patterns)] _ => return None , } ; Some (DynamicRelocationIterator { inner }) } # [cfg (not (feature = "elf"))] fn dynamic_relocations (& self) -> Option < DynamicRelocationIterator < 'data , '_ , R > > { None } fn symbol_map (& self) -> SymbolMap < SymbolMapName < 'data > > { with_inner ! (self , File , | x | x . symbol_map ()) } fn object_map (& self) -> ObjectMap < 'data > { with_inner ! (self , File , | x | x . object_map ()) } fn imports (& self) -> Result < Vec < Import < 'data > > > { with_inner ! (self , File , | x | x . imports ()) } fn exports (& self) -> Result < Vec < Export < 'data > > > { with_inner ! (self , File , | x | x . exports ()) } fn has_debug_symbols (& self) -> bool { with_inner ! (self , File , | x | x . has_debug_symbols ()) } # [inline] fn mach_uuid (& self) -> Result < Option < [u8 ; 16] > > { with_inner ! (self , File , | x | x . mach_uuid ()) } # [inline] fn build_id (& self) -> Result < Option < & 'data [u8] > > { with_inner ! (self , File , | x | x . build_id ()) } # [inline] fn gnu_debuglink (& self) -> Result < Option < (& 'data [u8] , u32) > > { with_inner ! (self , File , | x | x . gnu_debuglink ()) } # [inline] fn gnu_debugaltlink (& self) -> Result < Option < (& 'data [u8] , & 'data [u8]) > > { with_inner ! (self , File , | x | x . gnu_debugaltlink ()) } # [inline] fn pdb_info (& self) -> Result < Option < CodeView < '_ > > > { with_inner ! (self , File , | x | x . pdb_info ()) } fn relative_address_base (& self) -> u64 { with_inner ! (self , File , | x | x . relative_address_base ()) } fn entry (& self) -> u64 { with_inner ! (self , File , | x | x . entry ()) } fn flags (& self) -> FileFlags { with_inner ! (self , File , | x | x . flags ()) } }
    };
}

impl_126!();