macro_rules! deps {
    () => {
        Result!();
        XcoffSegmentIterator!();
        XcoffSection!();
        Import!();
        Export!();
        SectionIterator!();
        ObjectKind!();
        XcoffSectionIterator!();
        XcoffSymbol!();
        SymbolTable!();
        Architecture!();
        Comdat!();
        ComdatIterator!();
        DynamicRelocationIterator!();
        SegmentIterator!();
        SectionIndex!();
        SymbolIterator!();
        XcoffComdatIterator!();
        SymbolIndex!();
        Symbol!();
        FileHeader!();
        XcoffFile!();
        XcoffSymbolIterator!();
        XcoffComdat!();
        NoDynamicRelocationIterator!();
        Dynamic!();
        Section!();
        Object!();
        FileFlags!();
        Segment!();
        ReadRef!();
        XcoffSymbolTable!();
        XcoffSegment!();
    };
}

macro_rules! impl_780 {
    () => {
        deps!();
        impl < 'data , Xcoff , R > Object < 'data > for XcoffFile < 'data , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Segment < 'file > = XcoffSegment < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type SegmentIterator < 'file > = XcoffSegmentIterator < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type Section < 'file > = XcoffSection < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type SectionIterator < 'file > = XcoffSectionIterator < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type Comdat < 'file > = XcoffComdat < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type ComdatIterator < 'file > = XcoffComdatIterator < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type Symbol < 'file > = XcoffSymbol < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type SymbolIterator < 'file > = XcoffSymbolIterator < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type SymbolTable < 'file > = XcoffSymbolTable < 'data , 'file , Xcoff , R > where Self : 'file , 'data : 'file ; type DynamicRelocationIterator < 'file > = NoDynamicRelocationIterator where Self : 'file , 'data : 'file ; fn architecture (& self) -> Architecture { if self . is_64 () { Architecture :: PowerPc64 } else { Architecture :: PowerPc } } fn is_little_endian (& self) -> bool { false } fn is_64 (& self) -> bool { self . header . is_type_64 () } fn kind (& self) -> ObjectKind { let flags = self . header . f_flags () ; if flags & xcoff :: F_EXEC != 0 { ObjectKind :: Executable } else if flags & xcoff :: F_SHROBJ != 0 { ObjectKind :: Dynamic } else if flags & xcoff :: F_RELFLG == 0 { ObjectKind :: Relocatable } else { ObjectKind :: Unknown } } fn segments (& self) -> XcoffSegmentIterator < 'data , '_ , Xcoff , R > { XcoffSegmentIterator { file : self } } fn section_by_name_bytes < 'file > (& 'file self , section_name : & [u8] ,) -> Option < XcoffSection < 'data , 'file , Xcoff , R > > { self . sections () . find (| section | section . name_bytes () == Ok (section_name)) } fn section_by_index (& self , index : SectionIndex) -> Result < XcoffSection < 'data , '_ , Xcoff , R > > { let section = self . sections . section (index) ? ; Ok (XcoffSection { file : self , section , index , }) } fn sections (& self) -> XcoffSectionIterator < 'data , '_ , Xcoff , R > { XcoffSectionIterator { file : self , iter : self . sections . iter () . enumerate () , } } fn comdats (& self) -> XcoffComdatIterator < 'data , '_ , Xcoff , R > { XcoffComdatIterator { file : self } } fn symbol_table (& self) -> Option < XcoffSymbolTable < 'data , '_ , Xcoff , R > > { if self . symbols . is_empty () { return None ; } Some (XcoffSymbolTable { symbols : & self . symbols , file : self , }) } fn symbol_by_index (& self , index : SymbolIndex) -> Result < XcoffSymbol < 'data , '_ , Xcoff , R > > { let symbol = self . symbols . symbol (index) ? ; Ok (XcoffSymbol { symbols : & self . symbols , index , symbol , file : self , }) } fn symbols (& self) -> XcoffSymbolIterator < 'data , '_ , Xcoff , R > { XcoffSymbolIterator { file : self , symbols : self . symbols . iter () , } } fn dynamic_symbol_table < 'file > (& 'file self ,) -> Option < XcoffSymbolTable < 'data , 'file , Xcoff , R > > { None } fn dynamic_symbols (& self) -> XcoffSymbolIterator < 'data , '_ , Xcoff , R > { XcoffSymbolIterator { file : self , symbols : self . symbols . iter_none () , } } fn dynamic_relocations (& self) -> Option < Self :: DynamicRelocationIterator < '_ > > { None } fn imports (& self) -> Result < alloc :: vec :: Vec < Import < 'data > > > { Ok (Vec :: new ()) } fn exports (& self) -> Result < alloc :: vec :: Vec < Export < 'data > > > { Ok (Vec :: new ()) } fn has_debug_symbols (& self) -> bool { self . section_by_name (".debug") . is_some () || self . section_by_name (".dwinfo") . is_some () } fn relative_address_base (& self) -> u64 { 0 } fn entry (& self) -> u64 { if let Some (aux_header) = self . aux_header { aux_header . o_entry () . into () } else { 0 } } fn flags (& self) -> FileFlags { FileFlags :: Xcoff { f_flags : self . header . f_flags () , } } }
    };
}

impl_780!();