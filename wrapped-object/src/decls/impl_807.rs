macro_rules! deps {
    () => {
        Symbol!();
        FileAux!();
        Note!();
        SymbolIndex!();
        SymbolTable!();
        SymbolIterator!();
        Bytes!();
        Result!();
        StringTable!();
        CsectAux!();
        Pod!();
        Error!();
        U32Bytes!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! impl_807 {
    () => {
        deps!();
        impl < 'data , Xcoff , R > SymbolTable < 'data , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [doc = " Parse the symbol table."] pub fn parse (header : Xcoff , data : R) -> Result < Self > { let mut offset = header . f_symptr () . into () ; let (symbols , strings) = if offset != 0 { let symbols = data . read_slice (& mut offset , header . f_nsyms () as usize) . read_error ("Invalid XCOFF symbol table offset or size") ? ; let length = data . read_at :: < U32Bytes < _ > > (offset) . read_error ("Missing XCOFF string table") ? . get (BE) ; let str_end = offset . checked_add (length as u64) . read_error ("Invalid XCOFF string table length") ? ; let strings = StringTable :: new (data , offset , str_end) ; (symbols , strings) } else { (& [] [..] , StringTable :: default ()) } ; Ok (SymbolTable { symbols , strings , header : PhantomData , }) } # [doc = " Return the string table used for the symbol names."] # [inline] pub fn strings (& self) -> StringTable < 'data , R > { self . strings } # [doc = " Iterate over the symbols."] # [doc = ""] # [doc = " This does not return null symbols."] # [inline] pub fn iter < 'table > (& 'table self) -> SymbolIterator < 'data , 'table , Xcoff , R > { SymbolIterator { symbols : self , index : 0 , } } # [doc = " Empty symbol iterator."] # [inline] pub (super) fn iter_none < 'table > (& 'table self) -> SymbolIterator < 'data , 'table , Xcoff , R > { SymbolIterator { symbols : self , index : self . symbols . len () , } } # [doc = " Return the symbol entry at the given index and offset."] pub fn get < T : Pod > (& self , index : SymbolIndex , offset : usize) -> Result < & 'data T > { let entry = index . 0 . checked_add (offset) . and_then (| x | self . symbols . get (x)) . read_error ("Invalid XCOFF symbol index") ? ; let bytes = bytes_of (entry) ; Bytes (bytes) . read () . read_error ("Invalid XCOFF symbol data") } # [doc = " Get the symbol at the given index."] # [doc = ""] # [doc = " This does not check if the symbol is null, but does check if the index is in bounds."] fn symbol_unchecked (& self , index : SymbolIndex) -> Result < & 'data Xcoff :: Symbol > { self . get :: < Xcoff :: Symbol > (index , 0) } # [doc = " Get the symbol at the given index."] # [doc = ""] # [doc = " Returns an error for null symbols and out of bounds indices."] # [doc = " Note that this is unable to check whether the index is an auxiliary symbol."] pub fn symbol (& self , index : SymbolIndex) -> Result < & 'data Xcoff :: Symbol > { let symbol = self . symbol_unchecked (index) ? ; if symbol . is_null () { return Err (Error ("Invalid XCOFF symbol index")) ; } Ok (symbol) } # [doc = " Return a file auxiliary symbol."] pub fn aux_file (& self , index : SymbolIndex , offset : usize) -> Result < & 'data Xcoff :: FileAux > { debug_assert ! (self . symbol (index) ?. has_aux_file ()) ; let aux_file = self . get :: < Xcoff :: FileAux > (index , offset) ? ; if let Some (aux_type) = aux_file . x_auxtype () { if aux_type != xcoff :: AUX_FILE { return Err (Error ("Invalid index for file auxiliary symbol.")) ; } } Ok (aux_file) } # [doc = " Return the csect auxiliary symbol."] pub fn aux_csect (& self , index : SymbolIndex , offset : usize) -> Result < & 'data Xcoff :: CsectAux > { debug_assert ! (self . symbol (index) ?. has_aux_csect ()) ; let aux_csect = self . get :: < Xcoff :: CsectAux > (index , offset) ? ; if let Some (aux_type) = aux_csect . x_auxtype () { if aux_type != xcoff :: AUX_CSECT { return Err (Error ("Invalid index/offset for csect auxiliary symbol.")) ; } } Ok (aux_csect) } # [doc = " Return true if the symbol table is empty."] # [inline] pub fn is_empty (& self) -> bool { self . symbols . is_empty () } # [doc = " The number of symbol table entries."] # [doc = ""] # [doc = " This includes auxiliary symbol table entries."] # [inline] pub fn len (& self) -> usize { self . symbols . len () } }
    };
}

impl_807!()