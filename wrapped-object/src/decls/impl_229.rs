macro_rules! deps {
    () => {
        SymbolTable!();
        U32Bytes!();
        ImageAuxSymbolWeak!();
        SymbolMap!();
        Bytes!();
        SymbolIndex!();
        SymbolMapEntry!();
        ImageAuxSymbolFunction!();
        CoffHeader!();
        Result!();
        Note!();
        SymbolIterator!();
        ReadRef!();
        ImageSymbol!();
        StringTable!();
        Pod!();
        ImageAuxSymbolSection!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > , Coff : CoffHeader > SymbolTable < 'data , R , Coff > { # [doc = " Read the symbol table."] pub fn parse (header : & Coff , data : R) -> Result < Self > { let mut offset = header . pointer_to_symbol_table () . into () ; let (symbols , strings) = if offset != 0 { let symbols = data . read_slice (& mut offset , header . number_of_symbols () as usize) . read_error ("Invalid COFF symbol table offset or size") ? ; let length = data . read_at :: < U32Bytes < _ > > (offset) . read_error ("Missing COFF string table") ? . get (LE) ; let str_end = offset . checked_add (length as u64) . read_error ("Invalid COFF string table length") ? ; let strings = StringTable :: new (data , offset , str_end) ; (symbols , strings) } else { (& [] [..] , StringTable :: default ()) } ; Ok (SymbolTable { symbols , strings }) } # [doc = " Return the string table used for the symbol names."] # [inline] pub fn strings (& self) -> StringTable < 'data , R > { self . strings } # [doc = " Return true if the symbol table is empty."] # [inline] pub fn is_empty (& self) -> bool { self . symbols . is_empty () } # [doc = " The number of symbol table entries."] # [doc = ""] # [doc = " This includes auxiliary symbol table entries."] # [inline] pub fn len (& self) -> usize { self . symbols . len () } # [doc = " Iterate over the symbols."] # [inline] pub fn iter < 'table > (& 'table self) -> SymbolIterator < 'data , 'table , R , Coff > { SymbolIterator { symbols : self , index : SymbolIndex (0) , } } # [doc = " Return the symbol table entry at the given index."] # [inline] pub fn symbol (& self , index : SymbolIndex) -> Result < & 'data Coff :: ImageSymbol > { self . get :: < Coff :: ImageSymbol > (index , 0) } # [doc = " Return the auxiliary function symbol for the symbol table entry at the given index."] # [doc = ""] # [doc = " Note that the index is of the symbol, not the first auxiliary record."] # [inline] pub fn aux_function (& self , index : SymbolIndex) -> Result < & 'data pe :: ImageAuxSymbolFunction > { self . get :: < pe :: ImageAuxSymbolFunction > (index , 1) } # [doc = " Return the auxiliary section symbol for the symbol table entry at the given index."] # [doc = ""] # [doc = " Note that the index is of the symbol, not the first auxiliary record."] # [inline] pub fn aux_section (& self , index : SymbolIndex) -> Result < & 'data pe :: ImageAuxSymbolSection > { self . get :: < pe :: ImageAuxSymbolSection > (index , 1) } # [doc = " Return the auxiliary weak external symbol for the symbol table entry at the given index."] # [doc = ""] # [doc = " Note that the index is of the symbol, not the first auxiliary record."] # [inline] pub fn aux_weak_external (& self , index : SymbolIndex) -> Result < & 'data pe :: ImageAuxSymbolWeak > { self . get :: < pe :: ImageAuxSymbolWeak > (index , 1) } # [doc = " Return the auxiliary file name for the symbol table entry at the given index."] # [doc = ""] # [doc = " Note that the index is of the symbol, not the first auxiliary record."] pub fn aux_file_name (& self , index : SymbolIndex , aux_count : u8) -> Result < & 'data [u8] > { let entries = index . 0 . checked_add (1) . and_then (| x | Some (x .. x . checked_add (aux_count . into ()) ?)) . and_then (| x | self . symbols . get (x)) . read_error ("Invalid COFF symbol index") ? ; let bytes = bytes_of_slice (entries) ; Ok (match memchr :: memchr (b'\0' , bytes) { Some (end) => & bytes [.. end] , None => bytes , }) } # [doc = " Return the symbol table entry or auxiliary record at the given index and offset."] pub fn get < T : Pod > (& self , index : SymbolIndex , offset : usize) -> Result < & 'data T > { let bytes = index . 0 . checked_add (offset) . and_then (| x | self . symbols . get (x)) . read_error ("Invalid COFF symbol index") ? ; Bytes (bytes_of (bytes)) . read () . read_error ("Invalid COFF symbol data") } # [doc = " Construct a map from addresses to a user-defined map entry."] pub fn map < Entry : SymbolMapEntry , F : Fn (& 'data Coff :: ImageSymbol) -> Option < Entry > > (& self , f : F ,) -> SymbolMap < Entry > { let mut symbols = Vec :: with_capacity (self . symbols . len ()) ; for (_ , symbol) in self . iter () { if ! symbol . is_definition () { continue ; } if let Some (entry) = f (symbol) { symbols . push (entry) ; } } SymbolMap :: new (symbols) } }
    };
}

impl_229!();