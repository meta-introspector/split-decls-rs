macro_rules! deps {
    () => {
        Endian!();
        Sym!();
        VersionTable!();
        SymbolIndex!();
        HashTable!();
        HashHeader!();
        SymbolTable!();
        Result!();
        ReadRef!();
        Version!();
        FileHeader!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > HashTable < 'data , Elf > { # [doc = " Parse a SysV hash table."] # [doc = ""] # [doc = " `data` should be from an [`elf::SHT_HASH`] section, or from a"] # [doc = " segment pointed to via the [`elf::DT_HASH`] entry."] # [doc = ""] # [doc = " The header is read at offset 0 in the given `data`."] pub fn parse (endian : Elf :: Endian , data : & 'data [u8]) -> Result < Self > { let mut offset = 0 ; let header = data . read :: < elf :: HashHeader < Elf :: Endian > > (& mut offset) . read_error ("Invalid hash header") ? ; let buckets = data . read_slice (& mut offset , header . bucket_count . get (endian) as usize) . read_error ("Invalid hash buckets") ? ; let chains = data . read_slice (& mut offset , header . chain_count . get (endian) as usize) . read_error ("Invalid hash chains") ? ; Ok (HashTable { buckets , chains }) } # [doc = " Return the symbol table length."] pub fn symbol_table_length (& self) -> u32 { self . chains . len () as u32 } fn bucket (& self , endian : Elf :: Endian , hash : u32) -> SymbolIndex { SymbolIndex (self . buckets [(hash as usize) % self . buckets . len ()] . get (endian) as usize) } fn chain (& self , endian : Elf :: Endian , index : SymbolIndex) -> SymbolIndex { SymbolIndex (self . chains [index . 0] . get (endian) as usize) } # [doc = " Use the hash table to find the symbol table entry with the given name, hash and version."] pub fn find < R : ReadRef < 'data > > (& self , endian : Elf :: Endian , name : & [u8] , hash : u32 , version : Option < & Version < '_ > > , symbols : & SymbolTable < 'data , Elf , R > , versions : & VersionTable < 'data , Elf > ,) -> Option < (SymbolIndex , & 'data Elf :: Sym) > { let mut index = self . bucket (endian , hash) ; let mut i = 0 ; let strings = symbols . strings () ; while index != SymbolIndex (0) && i < self . chains . len () { if let Ok (symbol) = symbols . symbol (index) { if symbol . name (endian , strings) == Ok (name) && versions . matches (endian , index , version) { return Some ((index , symbol)) ; } } index = self . chain (endian , index) ; i += 1 ; } None } }
    };
}

impl_420!()